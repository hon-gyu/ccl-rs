

The parser parses the string into a list of key-value pairs, and then parses the values recursively.


model.mli
```ocaml
(** CCL type definitions and high-level functions to work with the parsed interface. *)

(** A helper module for a map from keys to values where the key is [string]. *)
module KeyMap : Map.S with type key = string

(** The actual type of the configuration. It's represented a dictionary from string to itself. *)
type t = Fix of t KeyMap.t

val compare : t -> t -> int

(** Self-explainable. *)
val empty : t

(** Merge two maps recursively. Keys from both Maps are preserved *)
val merge : t -> t -> t

(** [key_val key value] creates a singleton CCL [t] from the given [key]
associated with the [value]. *)
val key_val : string -> string -> t

(** An operator version of [key_val]. *)
val ( =: ) : string -> string -> t

(** [nested key entries] creates [t] with a key associated to multiple nested
values. *)
val nested : string -> t list -> t

(** [of_list maps] creates [t] by applying {!merge} to all entries. *)
val of_list : t list -> t

(** Convert a list of key-value pairs to the structured representation of map.

The way it works is that the function parses values with [Parser.parse_value]
recursively calls itself while parsing is possible. Thus, becoming a fixed point
over the list of key-value pairs. *)
val fix : Parser.key_val list -> t

(** Pretty-print the configuration. *)
val pretty : t -> string

(*
(** A module to construct CCL config values in pure OCaml without going through
the configuration. It uses the embeded Domain-Specific Language (eDSL) approach.

Useful for testing of defining default values.
*)
*)
```


parser.mli
```ocaml
(** A single key-value pair. Keys are strings and values are strings. Nothing extraordinary. *)
type key_val = {
  key : string;
  value : string;
}

type error = [ `Parse_error of string ]

(** Simple parsing function to parse the string representing key-value
configuration into the list of key-value pairs. *)
val parse : string -> (key_val list, [> error ]) result

(** Same as [parse] but with a slight modification: it calculates the biggest
prefix of spaces and considers all keys with this prefix (or less) to be
top-level. *)
val parse_value : string -> (key_val list, [> error ]) result
```