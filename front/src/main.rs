#![allow(non_snake_case)]

mod components;
mod models;

use components::{Header, Footer, PatientCard, PatientModal};
use dioxus::prelude::*;

use models::PatientModalVisibility;
use shared::models::Patient;
use uuid::Uuid;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

const API_ENDPOINT: &str = "api/v1";

fn patients_endpoint() -> String {
   let window = web_sys::window().expect("no global `window` exists");
   let location = window.location();
   let host = location.host().expect("should have a host");
   let protocol = location.protocol().expect("should have a protocol");
   let endpoint = format!("{}//{}/{}", protocol, host, API_ENDPOINT);
   format!("{}/patients", endpoint)
}

async fn get_patients() -> Vec<Patient> {
   log::info!("Fetching films from {}", patients_endpoint());
   reqwest::get(&patients_endpoint())
       .await
       .unwrap()
       .json::<Vec<Patient>>()
       .await
       .unwrap()
}

fn App(cx: Scope) -> Element {

    use_shared_state_provider(cx, || PatientModalVisibility(false));
    let is_modal_visible = use_shared_state::<PatientModalVisibility>(cx).unwrap();
    let patients = use_state::<Option<Vec<Patient>>>(cx, || None);
    let selected_patient = use_state::<Option<Patient>>(cx, || None);
    let force_get_patients = use_state(cx, || ());

    {
	let patients = patients.clone();

	use_effect(cx, force_get_patients, |_| async move {
	    let existing_patients = get_patients().await;
	    if existing_patients.is_empty() {
		patients.set(None);
	    } else {
		patients.set(Some(existing_patients));
	    }
	});
    }

    let delete_patient = move |patientId: Uuid| {
	let force_get_patients = force_get_patients.clone();
	cx.spawn({
            async move {
		let response = reqwest::Client::new()
                    .delete(&format!("{}/{}", &patients_endpoint(), patientId))
                    .send()
                    .await;
		match response {
                    Ok(_data) => {
			log::info!("Patient deleted");
			force_get_patients.set(());
                    }
                    Err(err) => {
			log::info!("Error deleting patient: {:?}", err);
                    }
		}
            }
	});
    };

    let create_or_update_patient = move |patient: Patient| {
	let force_get_patients = force_get_patients.clone();
	let current_selected_patient = selected_patient.clone();
	let is_modal_visible = is_modal_visible.clone();
	cx.spawn({
            async move {
		let response = if current_selected_patient.get().is_some() {
                    reqwest::Client::new()
			.put(&patients_endpoint())
			.json(&patient)
			.send()
			.await
		} else {
                    reqwest::Client::new()
			.post(&patients_endpoint())
			.json(&patient)
			.send()
			.await
		};
		match response {
                    Ok(_data) => {
			log::info!("Patient created");
			current_selected_patient.set(None);
			is_modal_visible.write().0 = false;
			force_get_patients.set(());
                    }
                    Err(err) => {
			log::info!("Error creating patient: {:?}", err);
                    }
		}
            }
	});
   };
    
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
                                       on_delete: move |_| {
                                           delete_patient(patient.id);
                                       }
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
	    patient: selected_patient.get().clone(),
	    on_create_or_update: move |new_patient| {
	        create_or_update_patient(new_patient);
	    },
	    on_cancel: move |_| {
		selected_patient.set(None);
		is_modal_visible.write().0 = false;
            }
	}
    })
}
