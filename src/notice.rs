use crate::config::NoticeConfig;

// time max value
const TIME_MAX_VALUE: i32 = 18000;
// repeat max value
const REPEAT_MAX_VALUE: i32 = 300;

/// notice node
pub struct NoticeNode {
    // occurs time
    pub timestamp: i32,
    // content
    pub text: String,
    // visible or not
    pub visible: bool,
}

/// gen notice node list
pub fn gen_notice_node(config: Vec<NoticeConfig>) -> Vec<NoticeNode> {
    let mut nodes = vec![];

    for c in &config {
        let start = c.start_time;
        let early_notice_time = c.early_notice_time;
        // negative number means no limit
        let mut end = c.end_time;
        if end < 0 || end > TIME_MAX_VALUE {
            end = TIME_MAX_VALUE;
        }
        let mut period = c.period;
        if period <= 0 || period > TIME_MAX_VALUE {
            period = TIME_MAX_VALUE;
        }
        let mut repeat_count = c.repeat_count;
        if repeat_count < 0 || repeat_count > REPEAT_MAX_VALUE {
            repeat_count = REPEAT_MAX_VALUE;
        }

        // gen count
        let mut gen_count = 0;
        for cur in (start..=end).step_by(period as usize) {
            if gen_count >= repeat_count {
                break;
            }

            // gen visible node
            let visible = NoticeNode {
                timestamp: cur,
                text: c.text.clone(),
                visible: true,
            };
            // gen invisible node, just notice
            if early_notice_time > 0 {
                let invisible = NoticeNode {
                    timestamp: cur - early_notice_time,
                    text: c.text.clone(),
                    visible: false,
                };
                nodes.push(invisible);
            }

            nodes.push(visible);
            gen_count += 1;
        }
    }

    // sort by timestamp
    nodes.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    return nodes;
}
