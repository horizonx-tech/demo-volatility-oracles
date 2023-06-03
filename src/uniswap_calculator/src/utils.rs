use std::ops::{Div, Mul};

use ic_cdk::api::time;
use ic_web3::types::U256;

fn pow10(n: U256) -> U256 {
    U256::from(10).pow(n)
}

pub fn calculate_exchange_rate(
    sqrt_price_x96: U256,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
) -> U256 {
    let token0_decimals = U256::from(token0_decimals);
    let token1_decimals = U256::from(token1_decimals);
    let precision = U256::from(precision);
    sqrt_price_x96
        .mul(sqrt_price_x96)
        .mul(pow10(precision))
        .div(U256::from(2).pow(U256::from(192)))
        .mul(pow10(token0_decimals))
        .div(pow10(token1_decimals))
}

pub fn calculate_realized_volatility(prices: &[U256]) -> f64 {
    let prices_len = prices.len();

    // Calculate the log-arithmic returns
    let mut all_squared_r: Vec<f64> = Vec::with_capacity(prices_len - 1);
    for i in 1..prices_len {
        let pt = prices[i].as_u128() as f64; // TODO: consider ovewflow
        let pt_minus_1 = prices[i - 1].as_u128() as f64; // TODO: consider ovewflow
        let r = (pt / pt_minus_1).ln().mul(100_f64);
        let r_squared = r.mul(r);
        all_squared_r.push(r_squared);
    }

    // Calculate the realized volatility
    let sum_of_squared_r = all_squared_r.iter().sum::<f64>();
    (sum_of_squared_r / prices_len as f64).sqrt() * (prices_len as f64).sqrt()
}

pub fn current_time_sec() -> u32 {
    (time() / (1000 * 1000000)) as u32
}

pub fn round_timestamp(timestamp: u32, unit: u32) -> u32 {
    timestamp / unit * unit
}

#[cfg(test)]
mod test_mod {
    use super::*;

    const ETHEREUM_USDCETH005_202303_SQRTPRICEX96: [u128; 31] = [
        1978057630279153570630795830076215,
        1941524372455752700280734097308328,
        1952091933429546329675048674390975,
        2000429466704162003787748359836129,
        2001900760398972652232442249128111,
        2003075265364622458925401458378203,
        2002071321252563431825818432456160,
        2004598383341359732080532417456327,
        2024243277835206269665929744640509,
        2090518582554613753266333259288742,
        2088225929099568179789437463578665,
        2020583331307105343990505413347210,
        1978265300564583135357665221837585,
        1932827022191447807246084119860715,
        1918261615064886909520598738473917,
        1947409869758673509438480591063283,
        1934266012540713962318026344236104,
        1871168873953970804126792969829144,
        1887025137979167155366331623875982,
        1875222604510862901715194665113652,
        1900196029632290114657169675175457,
        1863376121498133285267309680413004,
        1900202088374900935198847003810870,
        1858313201432468261581650000381103,
        1893186996136878889054301910702164,
        1897360780069752385437779254556524,
        1880206147354971623030916390224655,
        1912453024735044861582206790976835,
        1881234882793852033936145729808929,
        1871020692913375875588327315896547,
        1870569395896101347464491938479807,
    ];
    const ETHEREUM_USDCETH005_202303_EXCHANGE_RATE: [u128; 31] = [
        623330981979783,
        600518686448770,
        607073626842525,
        637510465809231,
        638448574426221,
        639197943231740,
        638557370001757,
        640170391988596,
        652779104210042,
        696223851107765,
        694697603255450,
        650420714983778,
        623461872117769,
        595150509773174,
        586214431269636,
        604165004918448,
        596037019151431,
        557784962304499,
        567278343011584,
        560204368943828,
        575224857464874,
        553148684940906,
        575228525660004,
        550146883098981,
        570989152865942,
        573509572013513,
        563185894633847,
        582669634172006,
        563802345959968,
        557696621925851,
        557427617452429,
    ];

    const ETHEREUM_USDCETH005_202304_SQRTPRICEX96: [u128; 30] = [
        1855712058371919227880374680547361,
        1856263764745701770056764981747552,
        1869788758497611233971922795226931,
        1862245954193022308918109577647899,
        1831235474446978573291656977492122,
        1812640959858896805311467075816445,
        1831057829124141590524713540032504,
        1834154836870436137744826670979979,
        1841832726509899909709234080730441,
        1836452286335257625398818277621450,
        1812280174646684759881887972015483,
        1821450361440143328121785070546952,
        1808533294477875934296290410530008,
        1765393089501814904782875822164448,
        1727810231620355921605992200267622,
        1731683441033806275730305853810254,
        1720806902166284718124682846370276,
        1738981584101650066877152501553836,
        1726831144723369965777691999155720,
        1800186130608333961849991999122479,
        1797220313129676313624887568369843,
        1842391258157642459064619007896237,
        1829716947594531104253312147732820,
        1836463678168996880401252174247866,
        1845864194102476668758042774815885,
        1833584350672105258031357770572060,
        1834054392926273118169229282777686,
        1813076843287931883635492680670842,
        1820781896804980708796383536546448,
        1813552718827319850211527495319638,
    ];
    const ETHEREUM_USDCETH005_202304_EXCHANGE_RATE: [u128; 30] = [
        548607843039015,
        548934095632532,
        556962456366603,
        552477901442622,
        534231163399564,
        523436991762533,
        534127518548220,
        535935867766842,
        540432182788880,
        537279327651081,
        523228644344266,
        528537143262670,
        521067335709433,
        496505057882222,
        475590220190701,
        477724858758555,
        471742616158301,
        481760066560144,
        475051373721750,
        516268533066076,
        514568823334031,
        540760002183810,
        533345523052672,
        537285993346456,
        542800605550436,
        535602530077925,
        535877170390475,
        523688762464263,
        528149272624226,
        523963702137345,
    ];

    #[test]
    fn test_calculate_realized_volatility_with_calculating_exchange_rate_03() {
        let prices: Vec<U256> = ETHEREUM_USDCETH005_202303_SQRTPRICEX96
            .iter()
            .map(|price| calculate_exchange_rate(U256::from(*price), 18, 8, 10))
            .collect();
        let realized_volatility = calculate_realized_volatility(&prices);
        println!("realized_volatility: {}", realized_volatility);
    }
    #[test]
    fn test_calculate_realized_volatility_with_calculating_exchange_rate_04() {
        let prices: Vec<U256> = ETHEREUM_USDCETH005_202304_SQRTPRICEX96
            .iter()
            .map(|price| calculate_exchange_rate(U256::from(*price), 18, 8, 10))
            .collect();
        let realized_volatility = calculate_realized_volatility(&prices);
        println!("realized_volatility: {}", realized_volatility);
    }
    #[test]
    fn test_calculate_realized_volatility_03() {
        let prices: Vec<U256> = ETHEREUM_USDCETH005_202303_EXCHANGE_RATE
            .iter()
            .map(|price| U256::from(*price))
            .collect();
        let realized_volatility = calculate_realized_volatility(&prices);
        println!("realized_volatility: {}", realized_volatility);
    }
    #[test]
    fn test_calculate_realized_volatility_04() {
        let prices: Vec<U256> = ETHEREUM_USDCETH005_202304_EXCHANGE_RATE
            .iter()
            .map(|price| U256::from(*price))
            .collect();
        let realized_volatility = calculate_realized_volatility(&prices);
        println!("realized_volatility: {}", realized_volatility);
    }
}

