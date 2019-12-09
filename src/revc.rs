
pub fn revc(input: &String) -> String {
    return input.chars()
        .rev()
        .filter(|x| {
            match x {
                'A' => true,
                'C' => true,
                'G' => true,
                'T' => true,
                _ => false,
            }
        })
        .map(
        |x| {
            match x {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => x,
            }
        }
    ).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revc_empty() {
        let input = String::new();
        assert_eq!(revc(&input), input);
    }

    #[test]
    fn revc_self_reverse_complement() {
        let input = String::from("AACCGGTT");
        assert_eq!(revc(&input), input)
    }

    #[test]
    fn revc_simple_complement() {
        let input = String::from("AA");
        assert_eq!(revc(&input), String::from("TT"));
    }

    #[test]
    fn revc_with_a_full_dataset() {
        let input = String::from("\
TTGAGCCTCGCGCAGTGTCTTTATCCGAGGGGGTCTTAGGGAATGGCTTGGCGTCTAACTCCAAATCGCCAGTTGGTCAA\
ATCGCTATAATAGTTGCCTCCACGAAGGTCGCAAGGACTGGGGGTAGATGCACGCTGACATGGAGCACAGTCGTCCACCA\
TAGATGCCATTACGCTAGCTCCGTAGATTAATCAGTCGGATGTTCACCAATACCCCATCATCTAGCGCTTTGGCGCGAGA\
CCCTACAAACTTTGCTTAGTCTTTTAGAGGATGATATTCAAACATTAAATATGTTCGATACATATACACTCTACAGTGTC\
CGTTCCGTGTGATGGGGTCCCTATTTTAATTTAAATATCTTCTGGCCAGGCGCAGCTCTACAGCCAGAGGTGAACATTTG\
GCAGAAATTCTAGTGAAGTGATGTCCCCCTTGAAGATAACTAAAGATATTATAAGCTTACGAGGAAGATGGTGAATTGGC\
GCTATATACCACTGGAACTGTCGGGACAATCCTAGGCCTCTGCGGAGACAACGATAATACATCCAATTAAATGCTCAGGG\
CTATCCTGGGCAGGCCTATGGGGCGGTGCCGATGTGAGACACAGCCTCGCGCTGATATGCAGGTTCAATGTTTTTAAAGA\
ATCCGACTACGCTCAAGGTAGACCATGGGGACTGGTCTTCCAAGCCTGAAGCGGGATCCATACATCTACCGTTCTGGCGT\
TCACTTGGCGCAGAGGTCTTTCCCGGTCTGACCCCGTGACCGAGAACTTTATGCCTGTCTGAATATACAGATTGACTGAC\
CATACTTCGATTACGGGCAGTCTTACCAAGATGAGATGCAATCACCATTTGGAACGAGAAGAGCATTGGGGACCCTAAAC\
A");
        let result =String::from("\
TGTTTAGGGTCCCCAATGCTCTTCTCGTTCCAAATGGTGATTGCATCTCATCTTGGTAAGACTGCCCGTAATCGAAGTAT\
GGTCAGTCAATCTGTATATTCAGACAGGCATAAAGTTCTCGGTCACGGGGTCAGACCGGGAAAGACCTCTGCGCCAAGTG\
AACGCCAGAACGGTAGATGTATGGATCCCGCTTCAGGCTTGGAAGACCAGTCCCCATGGTCTACCTTGAGCGTAGTCGGA\
TTCTTTAAAAACATTGAACCTGCATATCAGCGCGAGGCTGTGTCTCACATCGGCACCGCCCCATAGGCCTGCCCAGGATA\
GCCCTGAGCATTTAATTGGATGTATTATCGTTGTCTCCGCAGAGGCCTAGGATTGTCCCGACAGTTCCAGTGGTATATAG\
CGCCAATTCACCATCTTCCTCGTAAGCTTATAATATCTTTAGTTATCTTCAAGGGGGACATCACTTCACTAGAATTTCTG\
CCAAATGTTCACCTCTGGCTGTAGAGCTGCGCCTGGCCAGAAGATATTTAAATTAAAATAGGGACCCCATCACACGGAAC\
GGACACTGTAGAGTGTATATGTATCGAACATATTTAATGTTTGAATATCATCCTCTAAAAGACTAAGCAAAGTTTGTAGG\
GTCTCGCGCCAAAGCGCTAGATGATGGGGTATTGGTGAACATCCGACTGATTAATCTACGGAGCTAGCGTAATGGCATCT\
ATGGTGGACGACTGTGCTCCATGTCAGCGTGCATCTACCCCCAGTCCTTGCGACCTTCGTGGAGGCAACTATTATAGCGA\
TTTGACCAACTGGCGATTTGGAGTTAGACGCCAAGCCATTCCCTAAGACCCCCTCGGATAAAGACACTGCGCGAGGCTCA\
A");
        assert_eq!(revc(&input), result);
    }
}