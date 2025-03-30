# This is my attempt at making a compiler that can compile C code into a new proposed ISA architecture according to some research papers listed below.

# Frameworks/apps/stuff used:
- inkwell (for converting the code to IR)
- rust (for building the entire thing)
- lalrpop (to define my grammar rules)

Note: The backend for this is based on LLVM, but I am **NOT** modifying LLVM itself, i am simply translating from C code to ISA

# Research papers referenced
- [Flexible Instruction Set Architecture for Programmable Look-up Table based Processing-in-Memory](https://ieeexplore.ieee.org/document/9643747)
- [Towards Efficient LUT-based PIM: A Scalable and Low-Power Approach for Modern Workloads](https://arxiv.org/pdf/2502.02142)
- [PIM-AI: A Novel Architecture for High-Efficiency LLM Inference](https://arxiv.org/pdf/2411.17309)
- [ReApprox-PIM: Reconfigurable Approximate Lookup-Table (LUT)-Based Processing-in-Memory (PIM) Machine Learning Accelerator](https://ieeexplore.ieee.org/document/10443597)
