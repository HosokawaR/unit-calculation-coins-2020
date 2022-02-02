use super::read_csv::Record;

#[derive(Debug)]
pub enum FilterType {
    Code,
    Name,
}

#[derive(Debug)]
pub struct Filter {
    pub kind: FilterType,
    pub regex: String,
}

#[derive(Debug)]
pub struct Requirement<'a> {
    pub label: String,
    pub credit: f64,
    pub acquired_credit: f64,
    pub ok: bool,
    pub filter: Option<Filter>,
    pub followed_by: &'a str,
    pub children: Vec<Requirement<'a>>,
}

fn judge_part(requirement: &mut Requirement, records: &mut Vec<Record>) -> f64 {
    match &requirement.filter {
        Some(filter) => {
            let matched_records = records.iter_mut().filter(|record| {
                !record.is_read() && record.is_match(&filter) && record.is_acquired()
            });

            for record in matched_records {
                requirement.acquired_credit += record.credit;
                record.set_read(true);
            }

            requirement.ok = requirement.acquired_credit >= requirement.credit;
            requirement.acquired_credit
        }
        None => {
            let sum_credit = requirement
                .children
                .iter_mut()
                // TODO: 参照に変換する
                .map(|child| judge_part(child, records))
                .sum();

            requirement.acquired_credit = sum_credit;
            requirement.ok = requirement.acquired_credit >= requirement.credit;
            sum_credit
        }
    }
}

pub fn judge(requirement: &mut Requirement, records: &mut Vec<Record>) {
    judge_part(requirement, records);
}
