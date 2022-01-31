use super::read_csv::Record;

#[derive(Debug)]
pub enum FilterKind {
    Code,
    Name,
}

#[derive(Debug)]
pub struct Filter<'a> {
    pub kind: FilterKind,
    pub regex: &'a str,
}

#[derive(Debug)]
struct Requirement<'a> {
    label: &'a str,
    credit: f64,
    acquired_credit: f64,
    ok: bool,
    filter: Option<Filter<'a>>,
    followed_by: &'a str,
    children: Vec<&'a mut Requirement<'a>>,
}

fn judge_part(requirement: &mut Requirement, records: &mut Vec<Record>) -> f64 {
    match &requirement.filter {
        Some(filter) => {
            let result = records
                .iter_mut()
                .filter(|record| !record.is_read() && record.is_match(&filter));

            for record in result {
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
                .map(|child| judge_part(child, records))
                .sum();

            requirement.acquired_credit = sum_credit;
            requirement.ok = requirement.acquired_credit >= requirement.credit;
            sum_credit
        }
    }
}

pub fn judge(records: &mut Vec<Record>) {
    let mut a = Requirement {
        label: "線形代数A",
        credit: 2.0,
        acquired_credit: 0.0,
        ok: false,
        filter: Some(Filter {
            kind: FilterKind::Name,
            regex: "^線形代数A$",
        }),
        followed_by: "",
        children: vec![],
    };
    let mut b = Requirement {
        label: "線形代数B",
        credit: 2.0,
        acquired_credit: 0.0,
        ok: false,
        filter: Some(Filter {
            kind: FilterKind::Name,
            regex: "^線形代数B$",
        }),
        followed_by: "",
        children: vec![],
    };
    let mut c = Requirement {
        label: "微分積分A",
        credit: 2.0,
        acquired_credit: 0.0,
        ok: false,
        filter: Some(Filter {
            kind: FilterKind::Name,
            regex: "^微分積分A$",
        }),
        followed_by: "",
        children: vec![],
    };
    let mut d = Requirement {
        label: "必須科目",
        credit: 6.0,
        acquired_credit: 0.0,
        ok: false,
        filter: None,
        followed_by: "",
        children: vec![&mut a, &mut b, &mut c],
    };

    judge_part(&mut d, records);

    println!("{:#?}", d);
}
