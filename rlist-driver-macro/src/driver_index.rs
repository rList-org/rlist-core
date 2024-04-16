use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataEnum, Variant};

pub fn rlist_driver_index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let name_str = quote!(stringify!(#name));

    // check if the input is an enum
    match input.data {
        Data::Enum(_) => {}
        _ => {
            return TokenStream::from(
            quote!{
                compile_error!("rlist_driver_index can only be applied to enums");
            })
        }
    };

    // must be an enum
    let variants = match input.data  {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => unreachable!(),
    };


    let mut visitor_match_arms = Vec::new();
    // match driver.as_str() {
    //     "example_driver_1" => Ok(DriverIndex::ExampleDriver1(*config.downcast().unwrap())),  <--- this is the `visitor_match_arm`
    //     "example_driver_2" => Ok(DriverIndex::ExampleDriver2(*config.downcast().unwrap())),
    // }

    let mut deserializer_seed_match_arms = Vec::new();
    // match driver.as_str() {
    //      "onedrive" => OnedriveConfig::deserialize(deserializer).map(|c| Box::new(c) as Box<dyn std::any::Any>),     <--- this is the `deserializer_seed_match_arm`
    //      _ => Err(de::Error::custom("invalid driver")),
    // },

    let mut driver_enum_list = Vec::new();
    // items in the enum
    // #[rlist_driver_index]
    // pub enum DriverIndex {
    //     #[rlist_driver_name = "example_driver_1"]
    //     ExampleDriver1(ExampleDriver1),               <--- this is the item
    // }


    // every driver item should be like
    // #[rlist_driver_index]
    // pub enum DriverIndex {
    //     #[rlist_driver_name = "example_driver_1"]
    //     ExampleDriver1(ExampleDriver1),
    //     #[rlist_driver_name = "example_driver_2"]
    //     ExampleDriver2(ExampleDriver2),
    // }
    for Variant { ident, attrs, .. } in variants {
        let driver_name = attrs.iter()
            .find_map(|attr| {
                if attr.path().is_ident("rlist_driver_name") {
                    attr.parse_args::<syn::LitStr>().ok()
                } else {
                    panic!("Each variant must have a `rlist_driver_name` attribute")
                }
            });

        // match arms are for deserialization
        // #[rlist_driver_name = "example_driver_1"]
        //                        ^^^^^^^^^^^^^^^^
        //                        this is the driver name also used in the match arm
        let driver_name = driver_name.unwrap();

        // fill the `visitor_match_arms` and `deserializer_seed_match_arms`
        visitor_match_arms.push(quote! {
            #driver_name => Ok(#name::#ident(*config.downcast().unwrap())),
        });
        deserializer_seed_match_arms.push(quote! {
            #driver_name => #ident::deserialize(deserializer).map(|c| Box::new(c) as Box<dyn std::any::Any>),
        });

        // fill the `driver_enum_list`
        driver_enum_list.push(driver_name.value());
    }

    let helper = quote!{
        const FIELDS: &'static [&'static str] = &["driver", "config"];
        struct DriverIndexVisitor;
        struct ConfigDeserializer<'a> {
            driver: Option<&'a String>,
        }
    };

    let de_seed = quote!{
        impl<'de, 'a> DeserializeSeed<'de> for ConfigDeserializer<'a> {
            type Value = Box<dyn std::any::Any>;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: Deserializer<'de>,
            {
                match self.driver {
                    Some(driver) => match driver.as_str() {
                        #(#deserializer_seed_match_arms)*
                        _ => Err(de::Error::custom("invalid driver")),
                    },
                    None => Err(de::Error::custom("driver is required before config")),
                }
            }
        }
    };

    let visitor = quote!{
        impl<'de> Visitor<'de> for DriverIndexVisitor {
            type Value = #name;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct " + #name_str)
            }

            fn visit_map<A>(self, mut map: A) -> Result<DriverIndex, A::Error>
                where
                    A: MapAccess<'de>,
            {
                let mut driver = None;
                let mut config = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "driver" => {
                            if driver.is_some() {
                                return Err(de::Error::duplicate_field("driver"));
                            }
                            driver = Some(map.next_value::<String>()?);
                        },
                        "config" => {
                            if config.is_some() {
                                return Err(de::Error::duplicate_field("config"));
                            }
                            config = Some(map.next_value_seed(ConfigDeserializer { driver: driver.as_ref() })?);
                        },
                        _ => return Err(de::Error::unknown_field(&key, FIELDS)),
                    }
                }

                if let (Some(driver), Some(config)) = (driver, config) {
                    match driver.as_str() {
                        #(#visitor_match_arms)*
                        _ => Err(de::Error::custom("unknown driver")),
                    }
                } else {
                    Err(de::Error::missing_field("driver or config"))
                }
            }
        }
    };

    let impl_deserialize = quote!{
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
            {
                deserializer.deserialize_struct(#name_str, FIELDS, DriverIndexVisitor)
            }
        }
    };

    let expanded = quote!{
        #helper
        #de_seed
        #visitor
        #impl_deserialize
    };

    TokenStream::from(expanded)
}
