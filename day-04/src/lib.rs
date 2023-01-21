pub fn calculate_score_first(s: &str) -> u16 {
    s
     .lines()
     .map(|line|  {
        let line_vec: Vec<u16> = line
         .split([',','-'])
         .map(|value| {
            value.parse::<u16>().unwrap()
         })
         .collect();
        if (line_vec[0] <= line_vec[2] && line_vec[1] >= line_vec[3]) ||
           (line_vec[2] <= line_vec[0] && line_vec[3] >= line_vec[1]) {
            1
           } 
           else {
            0
           }
         })
     .sum()
    }

    pub fn calculate_score_second(s: &str) -> u16 {
        s
         .lines()
         .map(|line|  {
            let line_vec: Vec<u16> = line
             .split([',', '-'])
             .map(|value| {
                value.parse::<u16>().unwrap()
             })
             .collect();
            if (line_vec[1] < line_vec[2] && line_vec[0] < line_vec[3]) ||
               (line_vec[2] < line_vec[1] && line_vec[3] < line_vec[0]) {
                0
               } 
               else {
                1
               }
             })
         .sum()
        }
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = calculate_score_first(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_second() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = calculate_score_second(&input);
        assert_eq!(result, 4);
    }

}
