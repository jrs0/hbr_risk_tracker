#![allow(non_snake_case)]

mod components;
mod models;

use components::{Header, Footer, PatientCard, PatientModal};
use dioxus::prelude::*;

use models::PatientModalVisibility;
use shared::models::Patient;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {

    use_shared_state_provider(cx, || PatientModalVisibility(false));
    let is_modal_visible = use_shared_state::<PatientModalVisibility>(cx).unwrap();
    let patients = use_state::<Option<Vec<Patient>>>(cx, || None);
    let selected_patient = use_state::<Option<Patient>>(cx, || None);
    let force_get_patients = use_state(cx, || ());
    
    cx.render(rsx! {
       main {
           class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
           Header {}
           section {
               class: "md:container md:mx-auto md:py-8 flex-1",
               if let Some(patients) = patients.get() {
                   rsx!(
                       ul {
                           class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
                           {patients.iter().map(|patient| {
                               rsx!(
                                   PatientCard {
                                       key: "{patient.id}",
                                       patient: patient,
                                       on_edit: move |_| {
                                           selected_patient.set(Some(patient.clone()));
                                           is_modal_visible.write().0 = true
                                       },
                                       on_delete: move |_| {}
                                   }
                               )
                           })}
                       }
                   )
               }
	   }
	   Footer {}
       }
	PatientModal {
	    on_create_or_update: move |_| {},
	    on_cancel: move |_| {
		is_modal_visible.write().0 = false;
            }
	}
    })
}
