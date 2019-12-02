import Foundation

func fuel_cost(weight: Float) -> Int {
    let result = (weight/3.0) - 2
    if result > 0 {
        return Int(result)
    }
    return 0
}

func recursive_cost(weight: Float) -> Int {
    var cost = fuel_cost(weight: weight)
    var additional = fuel_cost(weight: Float(cost))
    while additional > 0 {
        cost += additional
        additional = fuel_cost(weight: Float(additional))
    }
    return cost
}

let path = "../advent_problems/day01/input"

if let handle = FileHandle.init(forReadingAtPath: path) {
    let inputString = String(data: handle.readDataToEndOfFile(), encoding: String.Encoding.utf8)!
    let numbers = inputString.split(separator: "\n").map({ Float($0)! })
    print(numbers.map(fuel_cost).reduce(0, +), numbers.map(recursive_cost).reduce(0, +))
} else {
    print("oh no!")
}