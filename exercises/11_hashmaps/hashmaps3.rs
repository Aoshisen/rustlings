// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.

        //使用迭代器版本 (推荐使用的版本)
        // 如果tem_1_name 存在 在 scores 中,更新scores 中的 goals_scored 和 goals_conceded
        // 如果不存在,则插入一个新的 TeamScores 结构体,
        // team_2_name 同理
        [
            (team_1_name, team_1_score, team_2_score),
            (team_2_name, team_2_score, team_1_score),
        ]
        .iter()
        .for_each(|&(team_name, goals_scored, goals_conceded)| {
            scores
                .entry(team_name)
                .and_modify(|team_scores| {
                    team_scores.goals_scored += goals_scored;
                    team_scores.goals_conceded += goals_conceded;
                })
                .or_insert(TeamScores {
                    goals_scored,
                    goals_conceded,
                });
        });

        // // 使用循环版本代码
        // let teams = [
        //     (team_1_name, team_1_score, team_2_score),
        //     (team_2_name, team_2_score, team_1_score),
        // ];
        // for (team_name, goals_scored, goals_conceded) in teams {
        //     let modify = |team_scores: &mut TeamScores| {
        //         team_scores.goals_scored += goals_scored;
        //         team_scores.goals_conceded += goals_conceded;
        //     };
        //     let insert = TeamScores {
        //         goals_conceded,
        //         goals_scored,
        //     };
        //     scores.entry(team_name).and_modify(modify).or_insert(insert);
        // }
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
