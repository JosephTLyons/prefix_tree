# TODO
- Implement Display trait ?
- Implement non-recursive print?
- Clean up code heavily
- Run rustfmt
- Have code reviewed and clean up all the non-idiomatic things in here
- Include free-to-use txt files
- Remove mut from print input and anywhere else?
- Are the muts for before variable name / data type correct?  I dont really know
  the difference here.
- Add a file print output method
    - Can use redirection for the time being
- Try to use box over Rc (Moxian)
- Try not to explicitly match on Option and Result types, try to use the methods
  that are provided
- Cloning too much... alternatives?
- Add a test to read in many alphabetized words from a file, 5,000 or so, then,
  print them out.  Then compare each file
- Try to switch to generics over char, if possible.
