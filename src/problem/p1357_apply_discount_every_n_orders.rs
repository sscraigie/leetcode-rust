/**
 * [1357] Apply Discount Every n Orders
 *
 * There is a supermarket that is frequented by many customers. The products sold at the supermarket are represented as two parallel integer arrays products and prices, where the i^th product has an ID of products[i] and a price of prices[i].
 * When a customer is paying, their bill is represented as two parallel integer arrays product and amount, where the j^th product they purchased has an ID of product[j], and amount[j] is how much of the product they bought. Their subtotal is calculated as the sum of each amount[j] * (price of the j^th product).
 * The supermarket decided to have a sale. Every n^th customer paying for their groceries will be given a percentage discount. The discount amount is given by discount, where they will be given discount percent off their subtotal. More formally, if their subtotal is bill, then they would actually pay bill * ((100 - discount) / 100).
 * Implement the Cashier class:
 * 
 * 	Cashier(int n, int discount, int[] products, int[] prices) Initializes the object with n, the discount, and the products and their prices.
 * 	double getBill(int[] product, int[] amount) Returns the final total of the bill with the discount applied (if any). Answers within 10^-5 of the actual value will be accepted.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["Cashier","getBill","getBill","getBill","getBill","getBill","getBill","getBill"]
 * [[3,50,[1,2,3,4,5,6,7],[100,200,300,400,300,200,100]],[[1,2],[1,2]],[[3,7],[10,10]],[[1,2,3,4,5,6,7],[1,1,1,1,1,1,1]],[[4],[10]],[[7,3],[10,10]],[[7,5,3,1,6,4,2],[10,10,10,9,9,9,7]],[[2,3,5],[5,3,2]]]
 * Output
 * [null,500.0,4000.0,800.0,4000.0,4000.0,7350.0,2500.0]
 * Explanation
 * Cashier cashier = new Cashier(3,50,[1,2,3,4,5,6,7],[100,200,300,400,300,200,100]);
 * cashier.getBill([1,2],[1,2]);                        // return 500.0. 1^st customer, no discount.
 *                                                      // bill = 1 * 100 + 2 * 200 = 500.
 * cashier.getBill([3,7],[10,10]);                      // return 4000.0. 2^nd customer, no discount.
 *                                                      // bill = 10 * 300 + 10 * 100 = 4000.
 * cashier.getBill([1,2,3,4,5,6,7],[1,1,1,1,1,1,1]);    // return 800.0. 3^rd customer, 50% discount.
 *                                                      // Original bill = 1600
 *                                                      // Actual bill = 1600 * ((100 - 50) / 100) = 800.
 * cashier.getBill([4],[10]);                           // return 4000.0. 4^th customer, no discount.
 * cashier.getBill([7,3],[10,10]);                      // return 4000.0. 5^th customer, no discount.
 * cashier.getBill([7,5,3,1,6,4,2],[10,10,10,9,9,9,7]); // return 7350.0. 6^th customer, 50% discount.
 *                                                      // Original bill = 14700, but with
 *                                                      // Actual bill = 14700 * ((100 - 50) / 100) = 7350.
 * cashier.getBill([2,3,5],[5,3,2]);                    // return 2500.0.  7^th customer, no discount.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^4
 * 	0 <= discount <= 100
 * 	1 <= products.length <= 200
 * 	prices.length == products.length
 * 	1 <= products[i] <= 200
 * 	1 <= prices[i] <= 1000
 * 	The elements in products are unique.
 * 	1 <= product.length <= products.length
 * 	amount.length == product.length
 * 	product[j] exists in products.
 * 	1 <= amount[j] <= 1000
 * 	The elements of product are unique.
 * 	At most 1000 calls will be made to getBill.
 * 	Answers within 10^-5 of the actual value will be accepted.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/apply-discount-every-n-orders/
// discuss: https://leetcode.com/problems/apply-discount-every-n-orders/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct Cashier {
        false
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        
    }
    
    fn get_bill(&self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1357() {
    }
}
