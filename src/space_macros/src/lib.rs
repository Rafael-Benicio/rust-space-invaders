use syn::DeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;


#[proc_macro_derive(BoxCollision)]
pub fn default_box(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name=input.ident;

    let expanded = quote! {
        impl BoxCollision for #name {
            fn aabb_collision(&mut self, rect: &CollisionBody) {
                if (rect.right_side()) > self.fisic_body.left_side()
                    && (self.fisic_body.right_side()) > rect.left_side()
                    && (rect.botton_side()) > self.fisic_body.top_side()
                    && (self.fisic_body.botton_side()) > rect.top_side()
                {
                    self.fisic_body.is_colliding = true;
                }
            }

            fn collision_box(&self) -> (CollisionBody, EntityType) {
                (self.fisic_body.clone(), self.entity_type)
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(BaseGameFlow)]
pub fn default_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name=input.ident;

    let expanded = quote! {
        impl BaseGameFlow for #name {
            fn get_id(&self) -> Uuid {
                self.id
            }

            fn get_type(&self) -> EntityType {
                self.entity_type
            }
        }
    };

    TokenStream::from(expanded)
}

