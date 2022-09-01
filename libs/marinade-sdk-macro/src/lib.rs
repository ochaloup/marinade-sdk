use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Token, Path};
use syn_unnamed_struct::Meta;

/// Example of macro generation that generates
/// `impl InstructionData` and `Discriminator`
///
/// ```
/// use marinade_sdk_macro::MarinadeInstructionData;
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// #[derive(MarinadeInstructionData, BorshSerialize, BorshDeserialize)]
/// #[discriminator([1,2,3,4,5,6,7,8])]
/// pub struct TestData {
///   pub lamports: u64
/// }
/// ```
///
/// will generate
///
/// ```
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// #[derive(BorshSerialize, BorshDeserialize)]
/// pub struct TestData {
///   pub lamports: u64
/// }
///
/// // GENERATED CODE:
/// impl micro_anchor::InstructionData for TestData {}
/// impl micro_anchor::Discriminator for TestData {
///   const DISCRIMINATOR: [u8; 8] = ([1,2,3,4,5,6,7,8]);
/// }
/// ```
#[proc_macro_derive(MarinadeInstructionData, attributes(discriminator))]
pub fn derive_instruction_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, attrs, .. } = &input;

    // token stream to be used as return value
    let mut output = proc_macro2::TokenStream::new();

    // Data struct requires implementing InstructionData trait
    let name = ident;
    let expanded = quote! {
        impl micro_anchor::InstructionData for #name {}
    };
    output.extend(expanded);

    // discriminator attribute is required
    let discriminator_attrs = attrs
        .iter()
        .filter(|a| a.path.is_ident("discriminator"))
        .flat_map(|attr| attr.tokens.clone())
        .collect::<Vec<_>>();
    if let Some(discriminator_attr) = discriminator_attrs.get(0) {
        let discriminator_impl = quote! {
            impl micro_anchor::Discriminator for #name {
                const DISCRIMINATOR:[u8;8] = #discriminator_attr;
            }
        };
        output.extend(discriminator_impl);
    } else {
        panic!("Discriminator attribute is required for macro MarinadeInstructionData, as parameter required [u8;8].")
    }

    output.into()
}

const AM_READ_ONLY: &str = "solana_program::instruction::AccountMeta::new_readonly({}, false)";
const AM_READ_ONLY_SIGNER: &str =
    "solana_program::instruction::AccountMeta::new_readonly({}, true)";
const AM_MUT: &str = "solana_program::instruction::AccountMeta::new({}, false)";
const AM_MUT_SIGNER: &str = "solana_program::instruction::AccountMeta::new({}, true)";

struct AccountsFieldData {
    name: String,
    is_pubkey: bool,
    field_type_name: proc_macro2::TokenStream,
    account_meta_formatter: String,
    signer: bool,
    mutate: bool,
}

impl AccountsFieldData {
    fn new(name: String, account_meta_formatter: String, is_pubkey: bool, field_type_name: proc_macro2::TokenStream) -> Self {
        AccountsFieldData {
            name,
            is_pubkey,
            field_type_name,
            account_meta_formatter,
            signer: false,
            mutate: false,
        }
    }
}

/// Example of macro generation that will generate a new struct `TestAccountInfos`
/// and all micro anchor implementations required for the instruction would work.
///
/// ```
/// use marinade_sdk_macro::{MarinadeInstructionAccounts, MarinadeInstructionData};
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// // TestData is reequired as this macro depends on existent of it, based on its name!
/// #[derive(MarinadeInstructionData, BorshSerialize, BorshDeserialize)]
/// #[discriminator([1,2,3,4,5,6,7,8])]
/// pub struct TestData {
///   pub lamports: u64
/// }
///
/// #[derive(MarinadeInstructionAccounts)]
/// #[ownerid(solana_program::bpf_loader::ID)]
/// pub struct TestAccounts {
///     #[account(mut)]
///     pub marinade: solana_program::pubkey::Pubkey,
/// }
/// ```
///
/// ```
/// use marinade_sdk_macro::MarinadeInstructionData;
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// #[derive(MarinadeInstructionData, BorshSerialize, BorshDeserialize)]
/// #[discriminator([1,2,3,4,5,6,7,8])]
/// pub struct TestData {
///   pub lamports: u64
/// }
///
/// pub struct TestAccounts {
///     pub marinade: solana_program::pubkey::Pubkey,
/// }
///
/// // GENERATED CODE:
/// impl micro_anchor::Owner for TestAccounts {
///     fn owner() -> solana_program::pubkey::Pubkey {
///         solana_program::bpf_loader::ID
///     }
/// }
/// pub struct TestAccountInfos<'info> {
///     pub marinade: solana_program::account_info::AccountInfo<'info>,
/// }
/// impl<'info> micro_anchor::Owner for TestAccountInfos<'info> {
///     fn owner() -> solana_program::pubkey::Pubkey {
///         solana_program::bpf_loader::ID
///     }
/// }
/// impl<'info> From<&TestAccountInfos<'info>> for TestAccounts {
///     fn from(
///         TestAccountInfos {
///             marinade,
///         }: &TestAccountInfos<'info>,
///     ) -> Self {
///         Self {
///             marinade: marinade.key.clone(),
///         }
///     }
/// }
/// impl<'info> micro_anchor::ToAccountInfos<'info> for TestAccountInfos<'info> {
///     fn to_account_infos(
///         &self,
///     ) -> Vec<solana_program::account_info::AccountInfo<'info>> {
///             vec![self.marinade.clone()]
///     }
/// }
/// impl micro_anchor::ToAccountMetas for TestAccounts {
///     fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
///         vec![solana_program::instruction::AccountMeta::new(self.marinade, false)]
///     }
///     type Data = TestData;
/// }
/// impl<'info> micro_anchor::ToAccountMetas for TestAccountInfos<'info> {
///     fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
///         vec![solana_program::instruction::AccountMeta::new(self.marinade.key.clone(), false)]
///     }
///     type Data = TestData;
/// }
/// ```
#[proc_macro_derive(MarinadeInstructionAccounts, attributes(account, ownerid))]
pub fn derive_instruction_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput {
        ident,
        attrs,
        data,
        ..
    } = &input;

    let struct_name = ident.clone();

    // ownerid attribute is required
    let owner_ids = attrs
        .iter()
        .filter(|a| a.path.is_ident("ownerid"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated).expect("Could not parse 'ownerid' attribute")
        })
        .collect::<Vec<_>>();
    if owner_ids.len() != 1 {
        panic!("'ownerid' attribute is required for macro MarinadeInstructionAccounts, one parameter of kind identifier of type Pubkey is required")
    }
    let owner_id = owner_ids.get(0).unwrap();

    let struct_name_as_string = struct_name.to_string();
    let infos_struct_name_as_string = struct_name_as_string.strip_suffix("Accounts");
    if infos_struct_name_as_string.is_none() {
        panic!(
            "Struct {} annotated with MarinadeInstructionAccounts is expected to have a name ending with 'Accounts'",
            struct_name
        );
    }
    let infos_struct_name = proc_macro2::Ident::new(
        format!("{}AccountInfos", infos_struct_name_as_string.unwrap()).as_str(),
        proc_macro2::Span::call_site(),
    );
    let data_struct_name = proc_macro2::Ident::new(
        format!("{}Data", infos_struct_name_as_string.unwrap()).as_str(),
        proc_macro2::Span::call_site(),
    );
    let obj = match data {
        syn::Data::Struct(obj) => obj,
        _ => panic!("Only structs supported in MarinadeInstructionAccounts macro"),
    };
    let struct_fields = obj
        .fields
        .iter()
        .map(|field| {
            let field_ident = field
                .ident
                .as_ref()
                .expect("Structs must contain named fields")
                .clone();
            // when pubkey we want to work with AccountMeta conversion, otherwise leaving it
            let is_pubkey = match &field.ty {
                syn::Type::Path(ty) => field.ty.to_token_stream().to_string().ends_with("Pubkey"),
                _ => false,
            };

            let mut field_data = AccountsFieldData::new(
                field_ident.to_token_stream().to_string(),
                AM_READ_ONLY.to_string(),
                is_pubkey,
                field.ty.to_token_stream(),
            );
            field
                .attrs
                .iter()
                .filter(|a| a.path.is_ident("account"))
                .flat_map(|attr| {
                    attr.parse_args_with(<Punctuated<Meta, Token![,]>>::parse_terminated)
                        .expect("Could not parse 'from' attribute")
                })
                .for_each(|meta| match meta {
                    Meta::Path(path) => match path.to_token_stream().to_string().as_str() {
                        "mut" => {
                            field_data.mutate = true;
                        }
                        "signer" => {
                            field_data.signer = true;
                        }
                        _ => panic!("Unrecognized attribute of field '{}'", field_data.name),
                    },
                    _ => panic!(
                        "Attribute for field {} contains unrecognized value",
                        field_data.name
                    ),
                });
            if field_data.signer && field_data.mutate {
                field_data.account_meta_formatter = AM_MUT_SIGNER.to_string();
            } else if field_data.mutate {
                field_data.account_meta_formatter = AM_MUT.to_string();
            } else if field_data.signer {
                field_data.account_meta_formatter = AM_READ_ONLY_SIGNER.to_string();
            }

            (field_ident, field_data)
        })
        .collect::<Vec<_>>();
    // preparation of identifiers and data to be quoted later
    let fields_declaration = struct_fields
        .iter()
        .map(|(field, _)| quote!(pub #field: solana_program::account_info::AccountInfo<'info>))
        .collect::<Vec<_>>();
    let fields_name = struct_fields
        .iter()
        .map(|(field, _)| quote!(#field))
        .collect::<Vec<_>>();
    let fields_declaration_cloning = struct_fields
        .iter()
        .map(|(field, _)| quote!(#field: #field.key.clone()))
        .collect::<Vec<_>>();
    let fields_cloning = struct_fields
        .iter()
        .map(|(field, _)| quote!(self.#field.clone()))
        .collect::<Vec<_>>();
    let fields_meta = struct_fields
        .iter()
        .map(|(_, f_data)| {
            let self_name = format!("self.{}", f_data.name);
            let account_meta_def: proc_macro2::TokenStream = f_data
                .account_meta_formatter
                .replace("{}", self_name.as_str())
                .parse()
                .unwrap();
            quote!(#account_meta_def)
        })
        .collect::<Vec<_>>();
    let fields_meta_cloning = struct_fields
        .iter()
        .map(|(_, f_data)| {
            let self_cloning = format!("self.{}.key.clone()", f_data.name);
            let account_meta_def: proc_macro2::TokenStream = f_data
                .account_meta_formatter
                .replace("{}", self_cloning.as_str())
                .parse()
                .unwrap();
            quote!(#account_meta_def)
        })
        .collect::<Vec<_>>();

    let output = quote! {
        impl micro_anchor::Owner for #struct_name {
            fn owner() -> solana_program::pubkey::Pubkey {
                #owner_id
            }
        }
        pub struct #infos_struct_name<'info> {
            #(#fields_declaration),*
        }
        impl<'info> micro_anchor::Owner for #infos_struct_name<'info> {
            fn owner() -> solana_program::pubkey::Pubkey {
                #owner_id
            }
        }
        impl<'info> From<&#infos_struct_name<'info>> for #struct_name {
            fn from(
                #infos_struct_name {
                    #(#fields_name),*
                }: &#infos_struct_name<'info>,
            ) -> Self {
                Self {
                    #(#fields_declaration_cloning),*
                }
            }
        }
        impl<'info> micro_anchor::ToAccountInfos<'info> for #infos_struct_name<'info> {
            fn to_account_infos(&self) -> Vec<solana_program::account_info::AccountInfo<'info>> {
                vec![
                    #(#fields_cloning),*
                ]
            }
        }
        impl micro_anchor::ToAccountMetas for #struct_name {
            fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
                vec![
                    #(#fields_meta),*
                ]
            }
            type Data = #data_struct_name;
        }
        impl<'info> micro_anchor::ToAccountMetas for #infos_struct_name<'info> {
            fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
                vec![
                    #(#fields_meta_cloning),*
                ]
            }
            type Data = #data_struct_name;
        }
    };

    TokenStream::from(output)
}
