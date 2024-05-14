use dioxus::prelude::*;
use shared::models::Patient;
use uuid::Uuid;
use chrono::prelude::*;

use crate::components::Button;
use crate::models::{ButtonType, PatientModalVisibility};

#[derive(Props)]
pub struct PatientModalProps<'a> {
    on_create_or_update: EventHandler<'a, Patient>,
    on_cancel: EventHandler<'a, MouseEvent>,
    #[props(!optional)]
    patient: Option<Patient>,
}

pub fn PatientModal<'a>(cx: Scope<'a, PatientModalProps>) -> Element<'a> {

    let is_modal_visible = use_shared_state::<PatientModalVisibility>(cx).unwrap();

    let draft_patient = use_state::<Patient>(cx, || Patient {
	name: "Joe Blogs".to_string(),
	gender: "male".to_string(),
	date_of_birth: Some(Utc.with_ymd_and_hms(1990, 1, 3, 0, 0, 0).unwrap()),
	id: Uuid::new_v4(),
	created_at: None,
	updated_at: None,
    });

    {
	let draft_patient = draft_patient.clone();
	use_effect(cx, &cx.props.patient, |patient| async move {
            match patient {
		Some(patient) => draft_patient.set(patient),
		None => draft_patient.set(Patient {
                    name: "".to_string(),
                    gender: "".to_string(),
                    date_of_birth: Some(Utc.with_ymd_and_hms(1995, 1, 1, 0, 0, 0).unwrap()),
                    id: Uuid::new_v4(),
                    created_at: None,
                    updated_at: None,
		}),
            }
	});
    }
    
    if !is_modal_visible.read().0 {
	return None;
    }
    
    cx.render(rsx!(
        article {
            class: "z-50 w-full h-full fixed top-0 right-0 bg-gray-800 bg-opacity-50 flex flex-col justify-center items-center",
            section {
                class: "w-1/3 h-auto bg-white rounded-lg flex flex-col justify-center items-center box-border p-6",
                header {
                    class: "mb-4",
                    h2 {
                        class: "text-xl text-teal-950 font-semibold",
                        "👤 Patient"
                    }
                }
                form {
                    class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Name"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter patient name",
			    value: "{draft_patient.get().name}",
                            oninput: move |evt| {
				draft_patient.set(Patient {
                                    name: evt.value.clone(),
                                    ..draft_patient.get().clone()
				})
                            }
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Gender"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter patient gender",
			    value: "{draft_patient.get().gender}",
                            oninput: move |evt| {
				draft_patient.set(Patient {
                                    gender: evt.value.clone(),
                                    ..draft_patient.get().clone()
				})
                            }
			}
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Date of birth"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "date",
			    value: "{draft_patient.get().date_of_birth_string()}",
                            oninput: move |evt| {
				let assumed_utc = format!("{0}T00:00:00.000Z", evt.value);
				draft_patient.set(Patient {
                                    date_of_birth: Some(DateTime::parse_from_rfc3339(&assumed_utc).unwrap().into()),
                                    ..draft_patient.get().clone()
				})
			    }
                        }
                    }
                }
                footer {
                    class: "flex flex-row justify-center items-center mt-4 gap-x-2",
                    Button {
                        button_type: ButtonType::Secondary,
                        onclick: move |evt| {
                            cx.props.on_cancel.call(evt)
                        },
                        "Cancel"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |evt| {
			    cx.props.on_create_or_update.call(draft_patient.get().clone());
			    draft_patient.set(Patient {
				name: "".to_string(),
				gender: "".to_string(),
				date_of_birth: Some(Utc.with_ymd_and_hms(1995, 1, 1, 0, 0, 0).unwrap()),
				id: Uuid::new_v4(),
				created_at: None,
				updated_at: None,
			    });
                        },
                        "Save patient"
                    }
                }
            }

        }
    ))
}
