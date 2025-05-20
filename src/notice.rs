use crate::config::NoticeConfig;

// max value
const MAX_VALUE: i32 = 600;

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
        // negative number means no limit
        let mut start = c.start_time;
        if start < 0 {
            start = MAX_VALUE;
        }
        let mut end = c.end_time;
        if end < 0 {
            end = MAX_VALUE;
        }
        let mut period = c.period;
        if period < 0 {
            period = MAX_VALUE;
        }
        let mut repeat_count = c.repeat_count;
        if repeat_count < 0 {
            repeat_count = MAX_VALUE;
        }
        let mut early_notice_time = c.early_notice_time;
        if early_notice_time < 0 {
            early_notice_time = MAX_VALUE;
        }

        // gen count
        let mut gen_count = 0;
        for cur in (start..=end).step_by(period as usize) {
            if gen_count > repeat_count {
                break;
            }

            // gen visible node
            let visible = NoticeNode {
                timestamp: cur,
                text: c.text.clone(),
                visible: true,
            };
            // gen invisible node, just notice
            let invisible = NoticeNode {
                timestamp: cur - early_notice_time,
                text: c.text.clone(),
                visible: false,
            };
            nodes.push(invisible);
            nodes.push(visible);
            gen_count += 1;
        }
    }

    // sort by timestamp
    nodes.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    return nodes;
}
