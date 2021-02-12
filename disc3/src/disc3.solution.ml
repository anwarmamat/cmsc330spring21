(* Type Inference Exercises *)

(* int -> int -> int *)
let f1 a b = a + b;;

(* bool -> bool -> bool *)
let f2 a b = if a then b else a;;

(* float -> float -> string -> string *)
let f3 a b c = if (a +. b) == 0.0 then "Hi" else c;;

(* Type Definition Exercises*)

let tf1 a = 
if a == "hi" then 1 else 0

let tf2 a b c = b == c

(* assume a and b are nonempty *)
let tf3 a b = 
match a, b with
| (h::t, h1::t1) -> if h == h1 then h else h1

(* Function Exercises *)

let concat str1 str2 = str1 ^ str2;;


let add_to_float integer flt = flt +. (float_of_int integer);;


let rec fib n =
    match n with
    |0 -> 0
    |1 -> 1
    |_ -> (fib (n - 1)) + (fib (n - 2));;


(* List and Tuple Exercises *)

let rec add_three lst =
    match lst with
    | [] -> []
    | h::t -> (h + 3)::(add_three t);;


let rec filter n lst =
    match lst with
    | [] -> []
    | h::t -> if (h <= n) then (h::(filter n t)) else (filter n t);;


let rec double lst = 
    match lst with
    |[] -> []
    |h::t -> h::h::(double t)
