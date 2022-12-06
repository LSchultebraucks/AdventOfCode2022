use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";

    let part1 = solve(read_input(&filename), 4);
    println!("Part 1: {}", part1);

    let part1 = solve(read_input(&filename), 14);
    println!("Part 1: {}", part1);
}

fn read_input(filename: &str) -> String {
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        return line.unwrap();
    }
    return String::new();
}

fn solve(word: String, cnt_distinct_markers: usize) -> i32 {
    let word_bytes = word.as_bytes();
    for index in 0..word.len() - cnt_distinct_markers {
        let mut set = HashSet::new();
        for offset in 0..cnt_distinct_markers {
            set.insert(char::from(word_bytes[index+offset]));
        }
        if set.len() == cnt_distinct_markers {
            return (index + cnt_distinct_markers) as i32;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::{solve, read_input};

    #[test]
    fn test_read_input() {
        let expected = "rgffbnnqvqhhmtmzzmmpmllcggsdswwvvwzzpptbppgngsnncwcnwcwnwgwrwrrnqnlqlccwggrcgrccjgcgwghgffjgjgrgmmsrrhchfcfdccjwwzdzcdcbcjjtfjjltlvvtstvttszsvsmmfccwcjwwzmzhhjvjpvjpppcwppdtdvtdvtvztzvzffdfqqgbgffrgrpgphhbcctssncsnncfcppvnpvnvfnfvnnfggtpplggwsggldllzvzrznncbbjpplbbrrnsrssmrsrhhmqmnqqhpqqnrqnrrtrnnrwrwhhpjhphlhvvgbgzztqtvvmssdshhqnhnjjvhjhqqbrqrrmrlrdllvrlvlccfvcffhfssgvsswhwqqfwfwhhcffjmmjrrbgrgjgrrvsrvvwmwsmmszzvmzvmmwmtthvvfbfllpjpwpwjpwpnpjpqqwtqtwwqpwqqrsqqcpqqssrgsrrjwwqmwmgwgrggmvmgmfgmffjfhjfjttcztccmtmmccnznhhcgcffmnfmmqjjhwwtccbzbhhwnhnsnmsnmnqqchhhnggbzbjzjzszsfsjfjvffnlffvrvddbhhqccdnnzwnznjnpnmppthhqddjhddqjqlqplqqjpjwppqttswsvwvfwflfplffnbfbwfwhhlhtlhtltzzmlmslmlwwgzwzrrwhhlnnrfnnmpppjpgpbbhpbhppbrppbhhzgzczqcqvqggwppcnnhnznvznzhhcpctptpjtppdhhprhrcrpcprpplrprzrnrfnrnlrlvrrfqqnnsqsqrssnrnvvvghvvbzzqppjsjnnrprwrrmqqtfqfwwrdrsddprddbdzbdbndbdjdljjrtjrrnzrzjrzzpssftfmfbflbbcpcsppwrppntnhthlthtbbsmsbmsmggpssnhnjjtsjjwssdjdwwppcbpcclmccwzzrggmgmnnwjnnqppjttvllmhhvcctptbpbdpdhdqqgggbnnqcchlhssltstllvqlvqvdvjdvvqvsvfvhfhlfftqffztzdttfvvzmvzznvnsnjjvqjvjljplpvvzlzhzghgddbbzbmbnztgthrnpsqhhdvprtpdftmqfvgjzgdvqwmvgwbczvbschqfhdvqcfnmbgmtqmlmsqcbfhshrzzbrtpgnwwtzgnjghzrlwhntprqhvshjcfvlnchccbtnswfnmpccdppqrqrhngvlwrpplbnpgzbzwtzwrsptmsmlndcfmbnqgvnqvzwpvrtfsrwfvfwdvplrfdddwcnwhzchmwfjsfvbtbrjchmgqwfvvmpzhqcbzhrcmjrzmgrtnzrcqdqdqnpwjctlhrjcphbbcvhvqnhtwjrvrbzfzfzzplshvrcchvtgtjcnhlzsttwdhcmdvrdlgsngvtzqsjrcptwwbgvfbsvgnmfhmvgqtfzbhhmdznjlsghhnlwzhvplfvlqzbrsjhdvrshjbnfqgpscbpzmnmmcsdbtwbwmsvfjwdvtctslcqfssrhczdptlrjbfzjctqrcbppcfcbqqzhhftdjchtscwgwcnpvrrjvnnwrqtrqmbgdfcpqhddnvdnmlmqcgsndwbcfvfrzsrflsnqrpmszqrdlshlcsfrmnsqmtrvjwqcllftscwrtmvcjsmrlqvdfjzgfdtswqzldqgjvhczpqrfbclnbwcjprjncvhgfmjhgfgnvfzfnvbbstgsgtspdhtfsncttlwmllbbvqftjtshfjlqtjwlvrbwmzfhdmcbhtqdzqtzmdljqprwhqwcvbtfqbpjwztgqrvlrqdwmqrqzvptsgjdqnqdfwfpjdglwgpdrcfrrzmpjmtbwwrqqcsnmphcqtthlnlzlfftrjmtjwwqrldcfjjclzrsqvltsfchfggcwbzbtpqcjfvgpwnwwqrdbdvjplgsgctdhqvttmtpmmjfcqdslhjgtdppbmzbrfbrrjncfdhlmjgdwjrmbgpcgctghbvphpgfvvfvtplqrhnjjhqntjvbsggvrvjgrwptcgqgrmjtprvhnvjsdsfqrqrrltlhvfsjsqpbwndbsnjzplcfqtfbdqdzdvnljcsmjnrmwwjzqwsjbdlclsmccbqwnlpltqhqhmthhrdjwlqwrbltghsvblqntvjqzpmmpqwrrwvhmtfqzhrmjfglwfpthhgbhdmrtprtqgqbdcfqlbbvmzmchzglmpzdvhpqchhclbcvrcmhtzqfzrplbvmdhghsttzgdjbgmmtvlqjsstgwlwchhwmflwbgljgszghvfdsngbtbcnhzmmdmsjqfbcsmgpjwjqgwqdlrgznnmhphbrzcgnqczpcbljjmhnnrmzlpqbswwcjpjqcrtdmgprpmmbprwlmpzcmwlnbzqjsgftrncvmjmjwvqgszsrmrczhjnwlghgndhbthwlhfbncrwqfgnshrmdfhwmfvllhvbmmllwfbgrnrttpzffzvcfjfbrmdqfcfrdqsltpbbttnqnbncfhwghfbgwrltbhwmmlpczsvvdnwhchgfplnjsttsqvsznwvlwzgfnhfrrgplsfwrwnbfhblwtjpldjmzrglhppgsjsgzssbbgrrbdmpwhgjjmlfnpdbvtntcwrvltgfnrgwttvvjhqljjvvnmztwgzcclgbjnjspzwnpgtgvhhhqbzrnvmdcljcdncvpnpdhrhqwvlhllbsrmjbzlgczwjsrvqrqfqmtjrnpbtdhsrfbhrgtgnrmlgcwcmrvrdfcqbfcqczjchgrjpzqfqqmcpvnvbrssfrfsrjmlngplqqsnclgvpfhjbzdppjftlgbmcgdbrvqrclvtdnzhhcmzlrdrgbrqrlvfcnvznnqwswvhdpbbjmsqzcrmjngzmdftjzsjvgfdtcvdwhnwwjlcgcmflzzpzpgzrvmptbjgrdjbzscrglhcjppdjdshnvvbsnnddtbwsdrpnbnlrtppcrzbbjrfzhbwgmjqtmgjvmrjgfdndlmqvhgwprsjnhcmrnfdfdzggjjzcccnsnwwbjzmsfjjfhclptfcwqjhnphpwzszjmsrtnbqpqsvcfcvrrgclwlbrqngdcdlzrdfpvbgtznjzdvjngrgswhjnrpgswtqflpgvnpbsvcrtgtzcbhdjpbjwnpdzcmprtzlppbqvpzrrlbssmwhhvqqpmwggnzwcfdnzccjrrncnvjqgbpstctwpqvhdsbcsjrpbzbdlwwvjngtbnvpthppglgsnrbwnvpzldhsgwqhclgwrdsvcfdclvdsbrhcfnbgrtqswstnjfmfrdgphpvgjfqzcwnpcghdhtflgdnhhnnnwrpnmppszgqdcqjqpnccfsqjsmwtqvzwfqvtwtthwswmcqqcqjwcgwgzgmnpzbfpzrqbvqhbhtsvvbppbmqzfnsmwppzvsctgcncztsqdfhnlwjjtvwcchvnphwpnpzqsjhlrcmnbthtqlfclnsfdtpwfzljdpfszvhgzdpzhbjtphbfbmwgfrfplntfqhgpbjzghmblzzhdcbptngqwrtfmvtbhphpvdwpswbscrwzhnvqfwlbwwqqgstgmbqlllspbhbpjmmrfhmjwzdnsdwmpvlrspgzmdjwqwcjpbgwspsghjdvrbbsbtgwptqdvvdhbhqbdhpbzdsdwsjdbjnztdqqrdhwhtpvcbblbmjgmbpqghdsfthzjffrdvldfsnpcsmnzwzcqlnvcrcbqnfcdrfbffrvhqsbqjnnbzsqvqqdqwmsvbqvtgmnnlthpngfsljqnrdhhzpmwsnqvrgdnvlbgnndfcpjfgmzqssvnnrwbmslcqpnhnnwzggsjvqcsqpfzjcmcppntmsdtfggzdncqbfjsqvbzgnnlsdbgjqffsmvnbqlsrjwdmcjbrsrpchnnhdtlcdhfdltmlvtrwjpzphtbhzzrlrwbbhhpntgbcfmphnbrjdhrhvmvhfglrncngjdsvbfrqtqzsgvlzjqzcqnwdcfgvnpsqpphwfsdvpgnchjnnjhsqgvlqcvhzzzcrsqcvrbsbjbdbgddlbb".to_string();
        let actual = read_input("input.txt");
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part1_examples() {
        let a = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let b = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let c = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let d = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(5, solve(a, 4));
        assert_eq!(6, solve(b, 4));
        assert_eq!(10, solve(c, 4));
        assert_eq!(11, solve(d, 4));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1100, solve(read_input("input.txt"), 4));
    }

    #[test]
    fn test_part2_examples() {
        let a = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let b = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let c = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let d = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let e = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(19, solve(a, 14));
        assert_eq!(23, solve(b, 14));
        assert_eq!(23, solve(c, 14));
        assert_eq!(29, solve(d, 14));
        assert_eq!(26, solve(e, 14));
    }
    #[test]
    fn test_part2() {
        assert_eq!(2421, solve(read_input("input.txt"), 14));
    }
}
