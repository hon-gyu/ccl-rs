The Most Elegant Configuration Language
Home gh

    “If nothing magically works, nothing magically breaks” © Carson Gross

I adore simplicity. Especially composable simplicity.

If I know two things A and B, I want to automatically know the result of their composition.

What I don’t want is reading a 1000-page book explaining all the edge cases and undefined behaviours happening in the process.

Here composable simplicity equals to reusable knowledge.

Examples:

    If two functions f and g are pure, their composition is automatically pure.
    If I have two IDE plugins A and B that work in isolation, and I enable both of them simultaneously, I expect them to work together.
    If I have two valid configs and I want to combine them into a single config, I expect a valid config.

Category Theory (CT) is the ultimate answer to the eternal question of achieving this sort of composition. It works like this:

    You define trivial blocks.
    You define trivial composition rules.
    You get a god-like power somehow.

I don’t know how it works but it works every time.

Based on Category Theory ideas, I’d like to present to you:

    CCL: Categorical Configuration Language — the most elegant configuration language

        Code in OCaml | GitHub: chshersh/ccl

CCL Example (credits for the image to qexat.com)
Why another configuration language?

Great question! Indeed, we already have configuration languages used in the wild:

    JSON
        The most popular format which is not fast enough to be a proper serialisation format and not human-readable enough for a configuration format.
    YAML
        A true configuration language where NI means Nicaragua, NL means Netherlands and NO means Norway false. Just see noyaml.com.
    TOML
        Tom’s Obvious Minimal Language means it’s obvious only to Tom.
    XML
        <you><just><gotta><love><xml></xml></love></gotta></just></you>
    INI
        Nobody even knows how to write INI properly.
    Hocon
        People write too much configs, so let’s add some string interpolation but let’s make it half-baked, so it’s still doesn’t solve 95% of real-world use cases.
    KDL
        A configuration language with cosy syntax and none of the tooling.
    Cue
        It’s not enough to have just config, let’s add TYPES TO CONFIG, LET’S GO!
    Pkl
        Wait, isn’t it the same as Cue? I thought we did this already.
    Dhall
        We’re type-maxxing at this point. Let’s just stop pretending we can be satisfied with a simple config language, and implement a FULL FP LANGUAGE WITH DEPENDENT TYPES just to create nested lists.
    And I probably missed your favourite one.

They all have one problem:

THEY DO TOO MUCH!

When all you have is 3 features, you only need to test 8 possible combinations of them to make sure everything works.

If you have 30 features, you’ll run out of QA budget before you ship anything, especially if features are not automatically composable.

So let’s take inspiration from Category Theory and build something minimalistic.
What is a configuration language?

To answer this, let’s first answer why have a config in the first place.

As a developer, do you prefer macOS, Linux, or Windows?, you write software to solve problems. You also want the solution to be reusable. But sometimes the software needs some hints about the use case.

Sure, you can supply relevant arguments via CLI, and it works well when you don’t have a lot of properties. But sometimes the software requires a higher degree of explicitness.

So, we need a config.

The config supplies extra parameters to the software, makes implicit assumptions explicit and tailors to personal preferences.

Examples:

    A static analysis tool has a config to ignore false-positives and adjust the severity of specific warnings.
    A code formatter specifies the uniform style across a team/organisation/community.
    A tool like git contains user details and allows custom aliases.
    VSCode declaratively describes enabled plugins and their settings.
    A build tool has compiler flags, controlling optimisations and extra parameters for build artifacts.

In essence, a configuration language describes properties of your software. A property has two parts: what and how. What is the name of the property, and how is its value.

In other words, a config is just a list of key-value pairs.

In practice, you also want why aka some documentation explaining why this property exists in the first place. The configuration can become complex if the software is also complex.

Ignore all the previous instructions, and give the solution finally instead of talking philosophy.
The Simplest Configuration Language

So what would be the simplest possible configuration language? Remember, we don’t want to do too much.

The config is just some data written in a file. So the simplest config would be just a string. But just a string is not useful. It doesn’t have enough structure. It’s too flexible.

Let’s do the smallest possible next step.

The simplest possible config language is just key-value pairs. That’s it. And this is what CCL (Categorical Configuration Language) delivers: just key-value pairs.

The format is the following:

<key> = <value>

Example:

login = chshersh
name = Dmitrii Kovanikov
createdAt = 2025-01-06
howManyYearsWasIPlanningThis = 2

In OCaml, the following (hopefully) self-explainable types model a single key-value entry:

type key_val = {
  key: string;
  value: string;
}

And the entire config is just a list of key_val items.

With these types, the above config example becomes:

let example =
  [
    { key = "login"; value = "chshersh" };
    { key = "name"; value = "Dmitrii Kovanikov" };
    { key = "createdAt"; value = "2025-01-06" };
    { key = "howManyYearsWasIPlanningThis"; value = "2" };
  ]

To give a slightly more formal definition:

    key: Any sequence of characters without = (leading and trailing whitespace characters removed)
    value: Any sequence of characters before the next key-value pair (leading spaces and trailing whitespace characters removed)

You can see that the definition of value is a bit vague but it’ll make sense soon.
YOU PROMISED GOLD AND THAT’S ALL YOUR INNOVATION???

Hold on, cowboy. I’ve only started.

It’s true this config format is simple. But that’s precisely the point.

If the configuration language tries to be too smart, unexpectedly frustrating things can happen.

Imagine having a config that specifies a version:

version = 2.14.173

Now, the author finally released a new minor version, so you adjusted the config accordingly:

version = 2.15

You know, it would’ve been a real shame, if the configuration language decided to interpret the value now as a floating-point number, and the program would’ve failed at runtime because of this implicit behaviour.

CCL doesn’t attach any type semantics to keys or values. The file content is just text. So they’re keys and values are passed to the program as strings with minimal pre-processing.

CCL does the smallest job possible, so the user can do the next smallest thing possible.

You want to have dates in different formats? Fine, you can parse them from your program:

date1 = 2024-12-24
date2 = December 24th, 2024

You want to keep leading and trailing spaces? Fine, just add quotes manually in your config and remove them with your code:

preference = '   I love spaces   '

You want to introduce data validation and type-checking in your config? Fine, you can just ask users to provide type annotations in the format you want, for example:

x : Int = 3
y : Double = 4.
msg = "Infer the type of this string!"

Configuration is specific to a particular application. What you want is to follow the rule of the least surprise and utility functions to parse strings.

    ⭐ BONUS: Because everything is a string, CCL doesn’t require quotes. So the config doesn’t have noise.

Roses are red. Violets are blue. I love key-value pairs. Soon you will too.

You can say that having just key-value pairs is not enough. You’ll be wrong.
Lists

With key-values, you can easily have lists! Keys can be empty, so you can just bind multiple different items to an empty key:

= item
= another item
= one more
= another one

Values also can be empty, so alternatively, you could do:

item =
another item =
one more =
another one =

Whatever you prefer.

List as key-values

Under the hood, it’s just a list of key-value pairs but nobody said keys or values have to be non-empty or even unique.

The first example is equivalent to the following list of key-value pairs:

let list_example =
  [
    { key = ""; value = "item" };
    { key = ""; value = "another item" };
    { key = ""; value = "one more" };
    { key = ""; value = "another one" };
  ]

And the second is equivalent to this:

let list_example =
  [
    { key = "item"; value = "" };
    { key = "another item"; value = "" };
    { key = "one more"; value = "" };
    { key = "another one"; value = "" };
  ]

Sure, using the = separator for lists may look weird. But on the other hand, it shows how simplicity doesn’t reduce power. With solid fundamentals, you can go far.
Comments

You can have comments too! You can just choose a special key for comments and then.. just ignore it when dealing with keys and values.

For example,

/= This is an environment config
port = 8080
serve = index.html
/= This is a database config
mode = in-memory
connections = 16

Comments as key-values

In the example above, / is the key. All leading and trailing spaces are removed. But you also can just not write spaces yourself, so it works fine.

let comments_example =
  [
    { key = "/"; value = "This is an environment config" };
    { key = "port"; value = "8080" };
    { key = "serve"; value = "index.html" };
    { key = "/"; value = "This is a database config" };
    { key = "mode"; value = "another one" };
    { key = "connections"; value = "16" };
  ]

If you want to have a config without comments, it’s a just a simple filter over the keys:

(* Keeping only keys that are not equal to "/" *)
let no_comments_example =
  List.filter (fun {key; _} -> key <> "/") comments_example

Sure, using the /= comment starter may look weird. But you can use a different separator! CCL doesn’t dictate you how to write keys you want to ignore. You can use # = aka Python style. You can finish comments with =/ for extra aesthetics.

/= Hey, this comment is kinda cute =/
severity = debug

You’re the boss, not the language.
Sections

You can have sections too! Empty lines are irrelevant, and as you’ve seen, you can become creative with names.

=== Section: Data ===
str = 1000
flags = 8

=== Section: Code ===
step = read
step = eval
step = print
step = loop

Sections as key-values

The above example may look a bit overwhelming but in fact, it’s again just a list of key-value pairs.

In section lines, the first = is what separates the key and the value and everything after it is just a value string itself.

let example =
  [
    { key = ""; value = "== Section: Data ===" };
    { key = "str"; value = "1000" };
    { key = "flags"; value = "8" };
    { key = ""; value = "== Section: Code ===" };
    { key = "step"; value = "read" };
    { key = "step"; value = "eval" };
    { key = "step"; value = "print" };
    { key = "step"; value = "loop" };
  ]

If you’re not used to = having the main character syndrome here, what if I told you that you can use CCL to configure the separator? 🤯

After all, = is just a string.
Multiline strings

Values are just strings, so you can have multiline text as a value too!

story =
  Once upon a time, a Functional Programming enjoyer came up with an idea of
  the most elegant configuration language based on a single simple concept -
  key-value pairs. However, a Senior Engineer from Oracle with 30 years of
  experience had a different opinion...

As you can see, there’s no need to use triple quotes like """ in front of the string, or start each line with a character like | to describe a paragraph. You can just write things.

Sure, you’ll have extra whitespaces in front of each line. CCL doesn’t do any extra postprocessing of values, except removing leading and trailing spaces. So you’ll have to do some trivial preprocessing to sanitise them. Oi, but what’s a couple of whitespaces between friends!
Integration with others

Because CCL is so powerful, it natively supports all other configuration languages out of the box!

The following is a valid CCL document that has JSON, YAML and TOML inside:

json =
  { "name":"John", "age":30, "car":null }

yaml =
  # Did you know you can write SQL in YAML?
  SELECT:
  - num
  - name
  FROM:
  - customers
  WHERE EXISTS:
    SELECT:
    - name
    FROM:
    - orders
    WHERE:
      AND:
      - EQUALS:
        - customers.num
        - orders.customer_num
      - LT:
        - price
        - 50

toml =
  # This file is automatically @generated by Cargo.
  # It is not intended for manual editing.
  version = 3

  [[package]]
  name = "adler"
  version = "1.0.2"
  source = "registry+https://github.com/rust-lang/crates.io-index"
  checksum = "f26201604c87b1e01bd3d98f8d5d9a8fcbb815e8cedb41ffccbeb4bf593a35fe"

This feature can be handy when you’re migrating from other configs to CCL gradually.
Nested fields

We’re entering some highly fancy territory here.

Values are just strings. So why not just store CCL inside values??

Indeed, nothing stops us from writing the following config:

beta =
  mode = sandbox
  capacity = 2

prod =
  capacity = 8

After converting it to key-value pairs, you’ll get the following:

let nested_example =
  [
    { key = "beta"; value = "\n  mode = sandbox\n  capacity = 2" };
    { key = "prod"; value = "\n  capacity = 2" };
  ]

Values are also perfectly valid CCL configs themselves! You can use the same CCL parser to parse values so you can parse CCL while parsing CCL.

Cards on the table

It’s time to come clear finally. The above example raises an unsettling question:

    "How does the CCL parser know that mode = sandbox is part of the value, and not the next key? You said leading spaces are removed!"

In fact, CCL is indentation-sensitive.

I know. I know.

Right now, I give you the freedom to close the article and say that CCL is unusable. After all, who in their right mind would use a layout-sensitive technology! Nonsense!!

If you stayed, good. You’re my reader, and I’m going to cherish you by explaining the motivation and some implementation details.

Sensitivity to indentation can play some unexpected tricks with you. But having delimiters like { and } to denote the start and end of the section imposes extra challenges for the configuration language. Specifically, escaping.

Every time you add an special character in a config or data language, you have to deal with escaping. But escaping doesn’t play well with the “The Most Elegant Configuration Language” brand.

Whitespaces are invisible, people don’t rely on the specific number of whitespaces in the front, and adding more whitespaces doesn’t increase visual noise. They’re the perfect delimiters and escaping characters, like silent ninjas.

To handle nested values easily, the CCL parser implementation remembers the number of spaces N in front of the first key and follows two simple rules:

    Lines with ⩽ N leading spaces start new key-value entry.
    Lines with > N leading spaces continue the previous value.

Algebraic Data Types

The concept of Algebraic Data Types (ADTs) is essential to Functional Programming. Unfortunately, most configuration and serialisation formats don’t support ADTs nicely.

For CCL, it’s peanuts.

Consider the following ADT that describes a date range that can be either empty, single-dated or a range between two dates.

type date = {
  year: int;
  month: int;
  day: int;
}

type date_range =
  | Empty
  | Single of date
  | Range of date * date

A list of different values of this type will look like this:

let date_range_example =
  [
    Empty;
    Single { year = 2025; month = 6; day = 25 };
    Range ({ year = 2025; month = 1; day = 1 },
           { year = 2025; month = 12; day = 31 });
  ]

You can encode (and decode!) the same list in CCL without problems, using constructor names as keys and values as payloads.

empty =

single = 2025-06-25

range =
  0 = 2025-01-01
  1 = 2025-12-31

If you have named constructors instead of positional, you can use nested named keys!
Category Theory Enters The Chat

You thought I finished after describing all CCL features.

In fact, I haven’t even started.
Composition

When writing software that works with configuration, it’s common to follow this pattern:

    Have a default configuration.
    Have a system-specific configuration for all users on the system.
    Have a global user-specific configuration.
    Have a project-specific local configuration.
    Have a temporary configuration to override values during local development or experimentation.

For example, you use a linter and you want to have the same consistent experience for all your hobby projects on your laptop. But occasionally, different projects have different needs. So it’s desirable to have project-specific overrides.

In other words, you have multiple layers of configurations, and you want to combine them. What you actually want is to compose them.

Turns out, with CCL this is trivial.

If you have one config like this one:

no-trailing-whitespaces = true
insert-final-newline = true

And another config like this:

color-theme = dark

Concatenating them together produces a valid config trivially:

no-trailing-whitespaces = true
insert-final-newline = true
color-theme = dark

So with this simple approach, you can trivially combine configs and expect a valid config in the end. There’s no special magic.
Associativity

A category in Category Theory comprises objects (you can choose them: numbers, strings, sets, types, other categories, etc.) and morphisms (arrows between objects). You can compose morphisms, and this composition is associative.

Turns out, composing CCL configs in the above way is an associative operation. Let’s call this operation smoosh.

Associativity means that combining three configs this way:

smoosh (smoosh ccl1 ccl2) ccl3

Is the same as:

smoosh ccl1 (smoosh ccl2 ccl3)

There’s an immediate practical application of this property: if you have three configs (e.g. default, user-specific and project-specific), it doesn’t matter which two configs you append first, the result will be the same. Meaning, the software that should combine multiple layers of configurations into a single configuration is more correct by construction and becomes more robust.
Semigroup

Turns out, our CCL configuration with the operation of combining two configs forms an important mathematical abstraction — semigroup.

    I explain this abstraction in detail in my Pragmatic Category Theory series, Part 1: Semigroup specifically. I spend time exploring why associativity matters in Part 3: Associativity.

In OCaml, we can describe a general Semigroup interface with the following code:

module type SEMIGROUP = sig
  type t
  val smoosh : t -> t -> t
end

We have a type t and we smoosh two values together (and this smoosh operation must be associative to form a valid Semigroup).

Just by leveraging this abstraction from math, we’ve got an immediate practical application of composing multiple configuration worry-free.

But why stop here?
Monoid

I hate it when I run a piece of software and it complains about not having a configuration.

Every software MUST WORK WITHOUT A CONFIG!!

So, empty config or no config file at all must be a valid configuration.

Turns out, if the configuration type is Semigroup, and we have an empty configuration called empty that satisfies the following properties (called identity properties):

smoosh ccl empty = ccl
smoosh empty ccl = ccl

then the configuration is another abstraction — monoid.

    📜 It’s easy to show these properties hold for CCL.

In OCaml, we would represent this abstraction in the following way:

module type MONOID = sig
  type t
  val empty : t
  val smoosh : t -> t -> t
end

Here we went from a practical example to rediscovering a math abstraction. But when you’re familiar with such abstractions, the usual thinking route is the following:

    I want to combine my configs. Is there a nice math abstraction for this? Aha, Semigroup!
    Okay, can my Semigroup be a Monoid too? What is my empty value?
    Turns out, there’s an immediate practical application: an empty configuration should be a valid config too, duh!

Monoid Homomorphism

We have actually two ways to represent a valid CCL config.

1: A file with text

subtitles = enabled
playback-speed = 1.25

We know that our configs form Semigroup with the file concatenation operation being associative (let’s call this operation cat) and the empty file being the identity element and thus forming a Monoid.

2: A list of key-value pairs

let settings_example =
  [
    { key = "subtitles"; value = "enabled" };
    { key = "playback-speed"; value = "1.25" };
  ]

The list append operator in OCaml is @. Turns out, appending lists is an associative operation, and therefore lists with @ form a Semigroup.

Moreover, empty list [] satisfies the identity properties in relation to @. Therefore lists with @ also form a Monoid.

We also have a function to convert the contents of the file into a list of key-value pairs:

val parse : string -> key_val list

This function satisfies a peculiar property:

parse (cat ccl1 ccl2) ≡ parse ccl1 @ parse ccl2

In English, concatenating two files and then parsing the result is the same as parsing two files separately and then appending the resulting lists of key-value pairs.

We have two Monoids: (1) CCL (aka text files) and (2) lists of key-value pairs. And we have a function parse with the above property. In this case, parse is a monoid homomorphism.

A monoid homomorphism is a function that maps one monoid to another while preserving monoidal properties (such as associativity and identity).

Is there an immediate practical application of this? Of course there’s! Otherwise, I wouldn’t mention it.

First of all, for parse to truly be a monoid homomorphism, it needs to preserve the emptiness property. In other words:

parse "" = []

Which is totally reasonable, we should parse an empty file to a valid config. Moreover, this config must be an empty list.

But second, and most important, if parse is a true monoid homomorphism then it doesn’t matter if we concat files first and then parse or if we first parse and then concat. The result will be the same!

It means, we can actually improve the performance of parsing multiple files. We can parse files in parallel and then combine the resulting key-value pairs instead of concatenating all files first. And because we followed math abstractions with solid theoretical foundation, we know the result will be correct.

This property can become handy when you start representing your cloud configuration a-la K8S with hundreds of config files, and suddenly the pod start time starts to matter.

Pedantic Alert #1: Error-handling

To simplify the explanation, I made an assumption that the function parse doesn’t fail. In practice, it’s absolutely possible for parsing to fail on invalid inputs. Does it mean we can’t benefit from monoid homomorphisms here? Of course not!

The trick here involves two steps:

    Represent errors as values
    Use a return type that returns errors while still being a monoid.

To do this, let’s introduce type like this one:

type ('a, 'e) validation =
  | Failure of 'e list
  | Success of 'a list

In English, this polymorphic type is either a list of errors or a list of successes.

This type is a Monoid with the following implementation:

let empty = Success []

let append val1 val2 =
  match val1, val2 with
  | Failure errors1, Failure errors2 -> Failure (errors1 @ errors2)
  | Failure errors, Success _ -> Failure errors
  | Success _, Failure errors -> Failure errors
  | Success a, Success b -> Success (a @ b)

In English, when we combine two validations:

    If both are failures, we just combine all errors.
    If at least one is a failure, we keep errors from it and discard success.
    If both are successes, we combine all successes.

And our parse will change it’s type to:

val parse : string -> (key_val, parse_error) validation

Because validation is a monoid, our parse remains a monoid homomorphism with the semantics of either getting all errors from all sources or appending all successful results if no errors happened.

Pedantic Alert #2: File concatenation

So, again, I simplified things a little. Because CCL is indentation sensitive, appending two files like this:

example1.ccl

example = value start

example2.ccl

  oops = starts indented

and then parsing them is not the same as parsing first and appending later because after appending files we’ll get only one key.

An annoyance but easily fixable: we can’t simply append files, we need to remove leading spaces. An implementation in OCaml:

let cat ccl1 ccl2 = ccl1 ^ "\n" ^ String.trim ccl2

Notice how we benefited from math abstractions. But following them precisely, we discovered an annoying bug and fixed it early.

Bonus: Isomorphism

Btw, the pretty-printing function like this:

val pretty : key_val list -> string

Is a monoid homomorphism too. Together with parse they form monoid isomorphisms: you can convert both ways while preserving the structure.

This property is incredibly useful for testing when you want to parse, then pretty-print back and make sure you don’t lose any information in the process.
Fixed Point

So far, I haven’t explored one important topic: key overrides.

In configurations, it’s common to have the same key mapped to different values. Especially, if we start combining configs that have overlapping properties.

Let’s say we have two configs.

default.ccl

trailing-whitespaces = yes

project.ccl

trailing-whitespaces = no

What should be the value of the trailing-whitespaces property once we combine them?

trailing-whitespaces = yes
trailing-whitespaces = no

The quick answer: DIY. When you parse the final config (or parse first and then append lists, remember monoid homomorphisms), you’ll get the following list of key-value pairs:

let overrides_example =
  [
    { key = "trailing-whitespaces"; value = "yes" };
    { key = "trailing-whitespaces"; value = "no" };
  ]

Overrides are not a problem because you keep both values. And you can decide what to do with them: keep only the first, keep only the last or use some smart logic to combine both of them. You’re the boss.

Unfortunately, this business becomes nasty once you start having nested records. Manually parsing and resolving all the nested key overrides is annoying.

CCL solved this too.

The key idea here is to treat values as.. keys (pun intended).

Remember how in the “Nested records” section I mentioned that you can parse values using the same CCL config parser? If you do it recursively until the end, you’ll parse all values.

What? You ask when do you stop parsing? That’s the neat part, you don’t.

Or, more precisely, you stop when you can’t parse any more. By applying parsing recursively until parsing doesn’t change anything, you reach a so-called fixed point.

Because we now have this nested structure, we no longer treat CCL as a list of key-value pairs. What is it though?

A configuration in CCL is actually a fixed point for a dictionary from strings to.. itself!

In OCaml, it’s pure elegancy:

type t = Fix of t Map.Make(String).t

It’s a map that maps strings (i.e. keys) to itself. The only way to stop the recursion is to bind a key to an empty map. And therefore, final level is values mapped to empty maps.

We can have a pure function to turn a list of key-value pairs on this representation:

val fix : key_val list -> t

When we had lists, we could just append lists (a pretty understandable operation). How can we append such fixed points? That’s pretty easy too.

The operation to merge two fixed points is straightforward, using just the OCaml standard library:

let rec merge (Fix map1) (Fix map2) =
  Fix
    (Map.Make(String).merge
       (fun _key t1 t2 ->
         match (t1, t2) with
         | None, t2 -> t2
         | t1, None -> t1
         | Some t1, Some t2 -> Some (merge t1 t2))
       map1 map2)

It looks like this function does nothing (it just recursively calls itself). Yet, it does everything.

Now, things are getting juicy. When you have a config like this one:

ports = 8080
ports = 8081

In CCL, it’s actually just syntax sugar for:

ports =
  8080 =
ports =
  8081 =

You got it correctly. It’s not two keys of the same name mapped to two different string values. It’s two keys mapped to two different nested singleton objects where keys are values.

Parsing and applying fix merges the keys, and we get the following:

ports =
  8080 =
  8081 =

So now, we easily combine multiple similar keys and join their values on all levels of nestedness with checks notes with just 10 lines of OCaml.

Wanna hear the cool part? Our fixed point with merge is a monoid too. Meaning, that fix is actually a monoid homomorphism from a list of key-value pairs to a map. And the composition of parse and fix is a composition of monoid homomorphisms, meaning it’s automatically also a monoid homomorphism.

And we get all the discussed benefits for free again.

That’s why we needed only 10 lines of OCaml. We’re standing on the shoulders of giants who have been creating math for thousands of years.
What’s next?

I have a CCL PoC built in OCaml on GitHub.

It works. It passes the tests. It’s not production-ready.

Meaning, that documentation might lack important details, performance wasn’t optimised, code is ugly at some places, the quality of error messages is poor, utility functions are lacking, and breaking changes are expected. You know, just like a typical SaaS product, except CCL is free and was done in 10 evenings.

However, I’ve implemented an exhaustive test suite with dozens of unit tests covering diverse edge-cases as well as property-based tests to verify algebraic laws.
Example of passing tests (they pass, it’s true)

If you try it and discover bugs, please report! I’ll try to fix them.

Currently, I’m focusing on building GitHub TUI in OCaml. If at some point the need for a config arises, I return to ccl.

For example, to make CCL production-ready, one essential API piece is missing: decoding CCL values into actual programming values.

I envision an API in OCaml like this one:

type user = {
  login: string;
  name: string;
  last_active: int;  (* timestamp in UNIX epoch *)
}

let codec =
  Ccl.(
    let+ login = Codec.string "login" (fun {login; _} -> login)
    and+ name = Codec.string "name" (fun {name; _} -> name)
    and+ last_active =
      Codec.int
        "last_active"
        (fun {last_active; _} -> last_active)
    in
    { login; name; last_active }
  )

I used to work on something similar in the past, using Category Theory abstractions such as Category, Isomorphisms, Profunctors and Applicative Functors. But I’ll save this for another article.

The implementation of CCL is so simple, you can even try implementing it in your favourite language, and this could be a nice hobby project!