// =======================================
// Problem
// https://app.codesignal.com/arcade/intro/level-5/veW5xJednTy4qcjso
// =======================================

// An IP address is a numerical label assigned to each device (e.g., computer, printer) participating in a computer network that uses the Internet Protocol for communication. There are two versions of the Internet protocol, and thus two versions of addresses. One of them is the IPv4 address.

// Given a string, find out if it satisfies the IPv4 address naming rules.

// Example

// For inputString = "172.16.254.1", the output should be
// isIPv4Address(inputString) = true;

// For inputString = "172.316.254.1", the output should be
// isIPv4Address(inputString) = false.

// 316 is not in range [0, 255].

// For inputString = ".254.255.0", the output should be
// isIPv4Address(inputString) = false.

// There is no first number.

// Input/Output

// [execution time limit] 2 seconds (rs)

// [input] string inputString

// A string consisting of digits, full stops and lowercase English letters.

// Guaranteed constraints:
// 1 ≤ inputString.length ≤ 30.

// [output] boolean

// true if inputString satisfies the IPv4 address naming rules, false otherwise.

fn is_ipv4_address(input_string: String) -> bool {
    let splitted = input_string.split('.').collect::<Vec<&str>>();

    if splitted.len() != 4 {
        return false;
    }

    let parsed = splitted.iter().map(|e| e.parse::<i32>().unwrap_or(-1));
    let stringer = parsed
        .clone()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(".");

    parsed.clone().all(|x| x >= 0 && x <= 255) && stringer == input_string
}

fn main() {
    let tests: Vec<(&str, bool)> = vec![
        ("1.1.1.1a", false),
        ("172.16.255.1", true),
        (".254.255.0", false),
        ("1.1.1.1.1", false),
        ("01.233.161.131", false),
        ("1", false),
    ];

    tests.iter().for_each(|(input, expect)| {
        let result = is_ipv4_address(input.to_string());
        assert_eq!(
            result, *expect,
            "expect that result {} = {} for {}",
            result, *expect, input
        );
    });
}
