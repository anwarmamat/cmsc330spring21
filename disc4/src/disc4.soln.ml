(*The following functions have been provided
  for your convenience*) 

let rec map f xs = match xs with
| [] -> []
| x :: xt -> (f x)::(map f xt)

let rec foldl f a xs = match xs with
| [] -> a
| x :: xt -> foldl f (f a x) xt

let rec foldr f xs a = match xs with
| [] -> a
| x :: xt -> f x (foldr f xt a) 

let rec rev lst = match lst with 
| [] -> [] 
| x :: xt -> (rev xt) @ [x]

(*Implement these functions here*)
let mul_thresh lst thresh = 
    foldl 
    (fun (less, greater) x ->
        if x < thresh then (x*less, greater) 
        else (less, x*greater)) 
    (1, 1) 
    lst


let multi_map f lst = 
    map (fun x -> map f x) lst 

type student_information = 
    { 
        name : string;
        age : int; 
        gpa : float;
    } 

let update_database lst = 
    map 
        (fun (name, age, gpa) -> 
            {name=name; age=age; gpa=gpa;}) 
        lst

let stalin_sort lst = 
    match lst with 
    | [] -> [] 
    | x :: xt -> 
        let (_, lst) = (foldl
        (fun (last_elem, lst) x -> 
            if x < last_elem then (last_elem, lst) 
            else (x, lst@[x])) 
        (x, [])
        lst) in 
        lst

let rec get_last_element lst = 
    match lst with 
    | [x] -> x 
    | _ :: xt -> get_last_element xt 
    | [] -> failwith "empty list has no elements"

let stalin_sort_right lst = 
    match lst with 
    | [] -> [] 
    | _ -> 
        let x = get_last_element lst in 
        let (_, lst) = (foldr 
        (fun x (last_elem, lst) -> 
            if x > last_elem then (last_elem, lst) 
            else (x, x::lst)) 
        lst 
        (x, [])) in 
        lst
