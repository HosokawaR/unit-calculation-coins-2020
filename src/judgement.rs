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
    pub limit_credit: f64,
    pub ok: bool,
    pub filter: Option<Filter>,
    pub order: Option<i64>,
    pub followed_by: &'a str,
    pub children: Vec<Requirement<'a>>,
}

fn judge_part(requirement: &mut Requirement, records: &mut Vec<Record>, prospect: &bool) -> f64 {
    match &requirement.filter {
        Some(filter) => {
            let matched_records = records.iter_mut().filter(|record| {
                !record.is_read() && record.is_match(&filter) && record.is_acquired(prospect)
            });

            for record in matched_records {
                requirement.acquired_credit += record.credit;
                record.set_read(true);
            }

            requirement.ok = requirement.acquired_credit >= requirement.credit;
            requirement.acquired_credit
        }
        None => {
            let children = &mut requirement.children;
            children.sort_by(|a, b| a.order.cmp(&b.order));

            let sum_credit = children
                // .into_iter()
                // .sorted_by(|a, b| a.order.cmp(&b.order))
                .iter_mut()
                // .into_iter()
                // TODO: 参照に変換する
                .map(|child| judge_part(child, records, prospect))
                .sum();

            requirement.acquired_credit = requirement.limit_credit.min(sum_credit);

            requirement.ok = requirement.acquired_credit >= requirement.credit;
            sum_credit
        }
    }
}

pub fn judge(requirement: &mut Requirement, records: &mut Vec<Record>, prospect: &bool) {
    judge_part(requirement, records, prospect);
}
