use regex::Regex;

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

const CATEGORIES: [&str; 5] = [
    "Basic Sciences",
    "Clinical Sciences",
    "Organ-based Basic & Clinical Sciences",
    "Clinical Subspecialties",
    "Special Problems or Issues in Anesthesiology",
];

const BASIC_SCIENCES_ITEMS: [&str; 28] = [
    "Abdominal compartment syndrome (A)",
    "Airway resistance: Poiseuille's law (B)",
    "Anes ventilator: Piston vs bellows (B)",
    "Arterial transducer: Position (B)",
    "ASRA Guidelines: Herbal suppl (B)",
    "Axillary blk: U/S anatomy (B)",
    "Brachial plexus block: Anatomy (B)",
    "Chi-squared test: Application (B)",
    "Codeine: Metabolism (A)",
    "Context sensitive half-time (B)",
    "Context sensitive halftime: Opioids (B)",
    "Dexmedetomidine: CV effects (B)",
    "Ketamine: Effect on BIS (A)",
    "L-spine: Radiologic anatomy (B)",
    "Local anesthetic: Side effects (B)",
    "Meyer Overton correlation (B)",
    "MH: Perioperative management (A)",
    "Muscle relaxants: Drug interactions (B)",
    "Neuromuscular blockade: Monitoring (B)",
    "Prolonged QT: Pharmacotherapy (A)",
    "Pseudocholinesterase deficiency (A)",
    "Renal insufficiency: ICU sedation (B)",
    "Sciatic nerve distribution (B)",
    "Soda lime exhaustion: Mgmt (B)",
    "Statistics: ANOVA (B)",
    "Statistics: Measures of dispersal (B)",
    "Turbulent flow: Gas density (B)",
    "Ultrasound physics (A)",
];

const CLINICAL_SCIENCES_ITEMS: [&str; 42] = [
    "Alveolar gas equation: Altitude (A)",
    "ASA physical status (B)",
    "ASA standards for monitoring (B)",
    "Bier block: Limitations (B)",
    "Complication bronchial blocker (B)",
    "Controlled hypotension: Monitoring (A)",
    "Crystalloid vs colloid: Side effects (B)",
    "Difficult airway: Mgmt (B)",
    "Difficult mask ventilation: Predictors (B)",
    "Emergence delirium: Anesthetic agents (B)",
    "Endobronchial intubation: Effects (B)",
    "Epidural anesth: Resp effects (B)",
    "Epidural local anes: GI effect (B)",
    "Foot surgery: Regional anesthesia (A)",
    "Hyperchloremic metabolic acidosis (B)",
    "Hypothermia: Physiol effects (A)",
    "Hypothermia: Prevention (B)",
    "Interscalene block: Sens distribution (A)",
    "Interscalene block: Side effects (A)",
    "Intraop glucose requirements (B)",
    "Intravascular fluids: Distribution (B)",
    "Knee surgery: Blocks (A)",
    "Lactated Ringer soln: Metab (B)",
    "Laryngospasm: Management (B)",
    "Levels of sedation: Definitions (B)",
    "Malignant hyperthermia: Treatment (B)",
    "Mallampati airway classification (B)",
    "Metoclopramide: Pharm effects (B)",
    "Neuromuscular blockade: Recovery (B)",
    "NMB: Brain stem reflexes (B)",
    "Occupational exposure: Radiation (B)",
    "PCA basal infusion & ped pts (B)",
    "Pediatric syndromes: Airway (B)",
    "Postop neuropathy: Lateral position (B)",
    "Postoperative nausea: Inhalational (B)",
    "Preoperative laboratory testing (B)",
    "RCRI: Major adverse cardiac event (B)",
    "Signs of emergence: GA (B)",
    "Spinal anes anatomy: Paramedian (B)",
    "Spinal baricity: Levels (B)",
    "TAP block: Anatomy (A)",
    "Tramadol: Pharm (B)",
];

const ORGAN_BASED_SCIENCES_ITEMS: [&str; 67] = [
    "ABG values: Measured vs calculated (A)",
    "Acromegaly: Airway (A)",
    "Acute intermittent porphyria: Trigger (A)",
    "Acute liver failure: CNS effects (A)",
    "Airway innervation (B)",
    "Amiodarone: Side effects (B)",
    "Anatomic shunt: Calculation (A)",
    "Anticholinesterase poisoning: Rx (A)",
    "Anticonvulsants: NMB duration (A)",
    "Arginine vasopressin: Secretion (B)",
    "Asthma Rx: IgE blockers (B)",
    "Blood brain barrier: Fluid transfer (B)",
    "Botulinum toxin: Mech of action (B)",
    "Carcinoid syndrome cardiac lesions (A)",
    "Cardiac anat: Coronary circ (B)",
    "Cardiac cycle: ECG (B)",
    "Cerebral blood vol: Vasodilators (B)",
    "Chest x-ray: Cardiac anatomy (A)",
    "Contrast-induced nephropathy (A)",
    "Corneal reflex: Anatomy (B)",
    "Coronary occlus: Resulting heart blk (B)",
    "Deep brain stimulation: Anatomy (A)",
    "Diabetes: Autonomic neurop signs (A)",
    "Digoxin toxicity: ECG (B)",
    "Diuretics: Potassium imbalance (B)",
    "Emergency transfusion: Compatibility (B)",
    "ERAS: Goal directed fluids (B)",
    "ERAS: Nutritional strategies (B)",
    "Evoked potentials: Aortic surgery (A)",
    "FENa: Significance (B)",
    "Fenoldopam: Mechanism (B)",
    "Hemophilia: Factor deficiencies (A)",
    "Hepatic blood flow: Regulation (B)",
    "HPV: Inhibition (B)",
    "Hypercalcemia: Acute treatment (A)",
    "Hypoglycemia: Glucagon (B)",
    "Hypokalemic periodic paralysis (A)",
    "Hypophosphatemia: Complications (A)",
    "IgA deficiency: Transfusion risk (B)",
    "IJ line: Cx (B)",
    "Lambert-Eaton: Pathophysiology (A)",
    "Leukotriene inh: Asthma (B)",
    "Lithotripsy: Contraindications (A)",
    "Mech of cerebral vasoconstr (B)",
    "Min invasive CABG: Single lung vent (A)",
    "Mitochondrial myopathy: Anes concerns (A)",
    "Myasthenia gravis and postop ventilat (A)",
    "Neostigmine: Side effects (B)",
    "Neuromuscular trans: Myasthenia (B)",
    "Nicotine: Anesth implications (B)",
    "Nitrous oxide: Air space expansion (A)",
    "NMJ: Ca release mechanism (B)",
    "NMJ: Postjunctional receptors (B)",
    "Postop hepatic dysfunction (B)",
    "Pregnancy: Fibrinogen (A)",
    "Pressure support: Weaning (A)",
    "Protamine reaction: Prevention (A)",
    "Pulm hypertension: Causes (A)",
    "Secondary hypothyroid: Labs (A)",
    "Spinal cord: Blood supply (B)",
    "Stored blood: Properties (B)",
    "Stress response: Hormones (B)",
    "Stress response: Lipolysis (B)",
    "SVR and PVR: Calculation (B)",
    "TEE: Atrial appendage (A)",
    "Total body water content (B)",
    "TRALI: Risk factors (B)",
];

const CLINICAL_SUBSPECIALTIES_ITEMS: [&str; 44] = [
    "ACLS: Special circumstances (A)",
    "Aging: Cardiac physiology (A)",
    "Aging: Phys changes (A)",
    "Airway fire prevention (A)",
    "Ambulatory surg: Discharge delay (A)",
    "Ambulatory surg: Fast track criteria (A)",
    "Ambulatory surgery: Patient selection (A)",
    "Aspirin toxicity and ABG (A)",
    "Cancer pain: Plexus block (A)",
    "Capnothorax: Dx (A)",
    "Catheter related sepsis: Prevention (A)",
    "Cocaine intoxication: Treatment (A)",
    "Cyanide toxicity: Rx (A)",
    "Emergency cesarean: Anes options (A)",
    "Fat embolism syndrome: Signs (A)",
    "FHT: Variable decelerations (A)",
    "Geriatric patients: Dosing difference (A)",
    "Geriatrics: MAC alterations (A)",
    "Glaucoma: Management of PONV (A)",
    "Hip arthroplasty: GA vs RA (A)",
    "Intraocular gas: Contraindications (A)",
    "Jet ventilation: Complications (A)",
    "Ketamine: Mechanism of analgesia (A)",
    "Laparoscopy: Complications (A)",
    "Laser: Safety (A)",
    "Mass casualty: Nerve agent (A)",
    "Near-drowning: Pathophys (A)",
    "Neonatal apnea hypoxemia physiol (A)",
    "Opioid overdose: ABG (A)",
    "Pediatric ETT size (A)",
    "Pediatric PONV: Risk factors (A)",
    "Pediatric sedation: Monitoring (A)",
    "Pregnancy: Hemodynamic changes (A)",
    "Pregnancy: Respiratory changes (A)",
    "Retro vs peribulbar block (A)",
    "Rhabdomyolysis: Complications (A)",
    "Rib fractures: Pain mgmt (A)",
    "Steep Trendelenburg: Risks (A)",
    "Tourniquet management (A)",
    "Tourniquet: Metabolic effects (A)",
    "Tumescent liposuction: Lidocaine (A)",
    "URI: Risk factors for complications (A)",
    "Uterotonic agents: Side effects (A)",
    "World Health Organization pain ladder (A)",
];

const SPECIAL_PROBLEMS_ITEMS: [&str; 14] = [
    "Biliary pressure: Drug effects (A)",
    "Contrast induced rxn: Risk factors (A)",
    "Core competencies (B)",
    "ECT: Seizure duration (A)",
    "Medication errors: Etiology (B)",
    "MRI compatibility: Implants (A)",
    "MRI: Quenching (A)",
    "Organ donor: Fluid mgmt (A)",
    "Organ transplant: Cold ischemia times (A)",
    "PACU bypass: Rationale (A)",
    "Physician impairment: Substance abuse (B)",
    "Professional liability: Tort law (A)",
    "Root cause analysis (A)",
    "Substance abuse: Relapse risk (B)",
];

fn main() {
    let path = env::args().skip(1).next().unwrap();
    let contents = read_to_string(path).unwrap();

    let name_re = Regex::new(
        r"Name: (?P<name>.+) Training Program: (?P<training_program>\d+) ID Number: (?P<id>\d+)",
    );

    let mut trainees: HashMap<u32, Trainee> = HashMap::new();

    let mut trainee: Option<&mut Trainee> = None;

    for line in contents.lines() {
        if let Some(caps) = name_re.captures(&line) {
            if let (name, training_program, id) = (
                caps.name("name"),
                caps.name("training_program"),
                caps.name("id"),
            ) {
                let id = id.as_str().parse();
                let name = name.as_str().to_string();
                trainee = Some(trainees.entry(id).or_insert(Trainee {
                    name,
                    id,
                    missed_topics: Vec::new(),
                }));
            }
        } else {

        }
    }
}

struct Trainee {
    name: String,
    id: u32,
    missed_topics: Vec<String>,
}
