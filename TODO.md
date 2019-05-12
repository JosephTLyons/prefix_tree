# TODO
- Implement Display trait ?
- Clean up code heavily
- Run format
- Remove print debug statements
- Have code reviewed and clean up all the non-idiomatic things in here
- Divide code into modules
- Include free-to-use txt files
- Add method to print all strings with a given prefix (just move down to that  
  level, then fall the recursive print on it)
- Remove mut from print input and anywhere else?
- Add a file print output method

1) You don't need Rc<RefCell<>> since any Letter can only ever be a level_below
   in not more than one other Letter struct. Rc would be useful if you could
   have more than one parent of a Letter.  Use Box instead.
2) You don't usually need to explicitly match on Option and Result types, since
   they have a bunch of methods that you could probably use instead

You don't really need to, but if you insist you can just mention me by the
username (it's the same on github).  I think all the situations where two
pointers point to the same thing are temporary and not really a problem for a
Box (also you are probably cloning stuff a bit more than you should, but that
probably doesn't matter much)
