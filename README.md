# High Bleeding Risk Tracker

This repository contains an app that tracks the dynamic bleeding risk of a patient as they present with an acute coronary syndrome, receive a PCI procedure, and spend time as an inpatient under observation, until they are discharged. At that point, their risk is only updated when new information is received about the patient.

The app is structured as a standalone fullstack deployment, intended to be distinct from other hospital systems. It maintains its own database of time-series patient data, which is updated by pulling data automatically from other hospital systems, or by manually inputting data by the clinician.

Bleeding risk is determined by simple calculations based on established risk scores, using data such as the patient age, haemoglobin, platelet count, renal function, and historical ICD-10/OPCS-4 coded patient data.

The primary design goals are:

* To be fast and easy to use.
* To clearly show what is being calculated, including what data may be missing.
* To show how the score changes over time.

Version one will not allow data to be manually inputted. This simplifies the app initially to one that does not require login by a user.

## User Interface

The user interface for the app should have these pages:

* Home page: allows searching for a patient by NHS number
* A patient page, with these items:
    * NHS number    
    * Age
    * Current value of the blood counts/other measurements, with a timestamp for each.
    * A graph showing the dynamic bleeding risk over time, in the current admission
*



