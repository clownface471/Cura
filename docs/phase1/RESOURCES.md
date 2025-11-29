# Phase 1 Learning Resources

Curated resources for kernel driver development with Rust.

## Books

### Essential (Must Read)

1. **The Rust Programming Language**
   - Authors: Steve Klabnik and Carol Nichols
   - Price: Free online
   - Link: https://doc.rust-lang.org/book/
   - Why: Foundation for everything else
   - Read: Chapters 1-16 minimum

2. **Windows Internals, Part 1 (7th Edition)**
   - Authors: Pavel Yosifovich, Mark Russinovich, et al.
   - Price: ~$50-60
   - Why: Bible of Windows kernel
   - Focus: Chapters 5 (Processes) and 8 (I/O System)
   - Note: Expensive but absolutely worth it

3. **Windows Kernel Programming**
   - Author: Pavel Yosifovich
   - Price: ~$40
   - Why: Practical driver development examples
   - Focus: Chapters 1-10
   - Note: Has C++ examples, you'll adapt to Rust

### Supplementary (Recommended)

4. **Programming Rust (2nd Edition)**
   - Authors: Jim Blandy, Jason Orendorff
   - Price: ~$50
   - Why: Deeper Rust concepts
   - When: After finishing Rust Book

5. **The Rustonomicon**
   - Free online: https://doc.rust-lang.org/nomicon/
   - Why: Unsafe Rust deep dive
   - When: Month 2-3
   - Warning: Advanced material

## Online Courses

### Rust

1. **Rustlings**
   - Link: https://github.com/rust-lang/rustlings
   - Type: Interactive exercises
   - Price: Free
   - Time: ~20 hours
   - When: Week 1-4

2. **Rust By Example**
   - Link: https://doc.rust-lang.org/rust-by-example/
   - Type: Code examples
   - Price: Free
   - Time: Self-paced
   - When: Alongside Rust Book

3. **Tour of Rust**
   - Link: https://tourofrust.com/
   - Type: Interactive tutorial
   - Price: Free
   - Time: ~5 hours
   - When: Week 1-2

### Windows Kernel

4. **OSR Seminars**
   - Link: https://www.osronline.com/seminars/
   - Type: Professional training
   - Price: $1000+ (expensive!)
   - When: Only if company-sponsored
   - Alternative: Free resources below

5. **Microsoft Learn - Driver Development**
   - Link: https://learn.microsoft.com/en-us/windows-hardware/drivers/
   - Type: Official documentation
   - Price: Free
   - When: Month 3-8

## Video Resources

### YouTube Channels

1. **Let's Get Rusty**
   - Focus: Rust tutorials
   - Quality: Excellent for beginners
   - Recommended: "Rust Crash Course" series

2. **Jon Gjengset**
   - Focus: Advanced Rust
   - Quality: Deep technical content
   - Recommended: "Crust of Rust" series
   - When: Month 2+

3. **fasterthanlime**
   - Focus: Rust internals
   - Quality: Very detailed
   - When: After Rust Book

### Specific Videos

4. **"Writing a Windows Kernel Driver"** by OSR
   - Search: OSR driver development YouTube
   - Free samples of their paid content

5. **"Rust for Windows Drivers"**
   - Search GitHub: windows-kernel-rs examples
   - Community demos

## Documentation

### Microsoft Official

1. **Filter Manager Documentation**
   - Link: https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/filter-manager-concepts
   - Critical: Read completely
   - When: Week 11-12

2. **Minifilter Design Guide**
   - Link: https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/filter-manager-and-minifilter-driver-architecture
   - When: Week 11-14

3. **Kernel-Mode Driver Architecture**
   - Link: https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/
   - When: Month 3-4

4. **IRQL Reference**
   - Link: https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/managing-hardware-priorities
   - Critical: Understand completely
   - When: Week 13-14

### Rust Kernel Development

5. **windows-kernel-rs**
   - Link: https://github.com/not-matthias/windows-kernel-rs
   - Type: Rust WDK bindings
   - When: Week 17+ (when you start coding driver)
   - Note: May need to adapt/fork for your needs

6. **rust-windows-kernel-driver**
   - Link: https://github.com/Trantect/win_driver_example
   - Type: Example driver
   - When: Week 17-20
   - Use: Reference implementation

## Community Resources

### Forums

1. **OSR Online Community**
   - Link: https://community.osr.com/
   - Focus: Windows driver development
   - Activity: Very active
   - Note: Professional developers, be respectful

2. **Rust Users Forum**
   - Link: https://users.rust-lang.org/
   - Focus: Rust language help
   - Activity: Extremely active

3. **Rust Discord**
   - Link: https://discord.gg/rust-lang
   - Focus: Real-time help
   - Channels: #beginners, #embedded (for no_std)

### Reddit

4. **r/rust**
   - Link: https://reddit.com/r/rust
   - Good for: Language questions
   - Activity: Daily posts

5. **r/kernel**
   - Link: https://reddit.com/r/kernel
   - Good for: Kernel concepts (Linux-focused but concepts apply)

6. **r/windows**
   - Link: https://reddit.com/r/windows
   - Good for: Windows-specific questions

## Tools & Software

### Development Environment

1. **Visual Studio 2022 Community**
   - Link: https://visualstudio.microsoft.com/
   - License: Free for individuals
   - Workloads: "Desktop C++" + "Driver Development"

2. **Windows Driver Kit (WDK)**
   - Link: https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk
   - Version: Match your Windows SDK
   - Required: Yes

3. **Rust (rustup)**
   - Link: https://rustup.rs/
   - Version: Latest stable
   - Required: Yes

4. **Git**
   - Link: https://git-scm.com/
   - Required: Yes (version control)

### Debugging Tools

5. **WinDbg Preview**
   - Source: Microsoft Store
   - Price: Free
   - When: Week 17+ (driver development)

6. **DebugView**
   - Link: https://learn.microsoft.com/en-us/sysinternals/downloads/debugview
   - Price: Free
   - When: Week 19+ (debug output)

7. **Process Monitor**
   - Link: https://learn.microsoft.com/en-us/sysinternals/downloads/procmon
   - Price: Free
   - Use: See file I/O in real-time

### Virtualization

8. **VMware Workstation Player** (Recommended)
   - Link: https://www.vmware.com/products/workstation-player.html
   - Price: Free for non-commercial
   - Why: Best debugging support

9. **Hyper-V** (Alternative)
   - Built into Windows 11 Pro
   - Price: Free
   - Why: Native, fast

10. **VirtualBox** (Budget Option)
    - Link: https://www.virtualbox.org/
    - Price: Free
    - Note: Debugging setup is harder

## Reference Materials

### Quick References

1. **Rust Cheat Sheet**
   - Link: https://cheats.rs/
   - Use: Quick syntax lookup
   - Bookmark: Yes

2. **Rust Standard Library Docs**
   - Link: https://doc.rust-lang.org/std/
   - Use: API reference
   - Note: `no_std` uses `core` instead

3. **Windows Driver Samples**
   - Link: https://github.com/microsoft/Windows-driver-samples
   - Use: Example drivers (in C)
   - When: Week 17+

### Checklists

4. **Driver Development Checklist** (OSR)
   - Search: "OSR driver development checklist"
   - Use: Verify you didn't forget anything

5. **IRQL Quick Reference**
   - Create your own based on Microsoft docs
   - Example:
     ```
     PASSIVE_LEVEL:   Can do anything
     DISPATCH_LEVEL:  No paging, no waiting
     HIGH_LEVEL:      Minimal code only
     ```

## Datasets & Samples

### Malware Datasets (Phase 2+)

1. **EMBER**
   - Link: https://github.com/elastic/ember
   - Type: Labeled malware dataset
   - Size: ~1M samples
   - When: Phase 2 (AI training)

2. **SOREL-20M**
   - Link: https://github.com/sophos/SOREL-20M
   - Type: Large malware dataset
   - Size: 20M samples
   - When: Phase 2 (AI training)

Note: These are for Phase 2, not Phase 1. Just know they exist.

## Blogs & Articles

### Must-Read Blog Posts

1. **"Writing a Windows Kernel Driver from Scratch"** (OSR)
   - Search: OSR kernel driver tutorial
   - When: Week 17

2. **"Understanding IRQL"** (OSR)
   - Search: OSR IRQL explained
   - When: Week 13-14

3. **"Filter Manager Internals"**
   - Search: Microsoft filter manager architecture
   - When: Week 11-12

4. **"Rust for Safety-Critical Systems"**
   - Search: Rust memory safety kernel
   - When: Month 1-2

### Blogs to Follow

5. **OSR Blog**
   - Link: https://www.osronline.com/blog/
   - Frequency: Weekly
   - Focus: Driver development

6. **This Week in Rust**
   - Link: https://this-week-in-rust.org/
   - Frequency: Weekly
   - Focus: Rust ecosystem

## Practice Challenges

### Coding Challenges

1. **Rustlings**
   - 100+ exercises
   - Week 1-4

2. **Exercism - Rust Track**
   - Link: https://exercism.org/tracks/rust
   - 100+ problems with mentorship
   - Week 1-8

3. **Advent of Code** (in Rust)
   - Link: https://adventofcode.com/
   - Practice problem-solving
   - When: Evenings/weekends

### Driver Challenges (Create Your Own)

4. **Challenge 1**: Driver that logs all file opens
5. **Challenge 2**: Driver that blocks .txt files
6. **Challenge 3**: Driver that counts process creations
7. **Challenge 4**: Driver that blocks specific registry keys

## Study Plan

### Month 1-2: Rust
- [ ] Read Rust Book (Chapters 1-16)
- [ ] Complete Rustlings (100%)
- [ ] Complete 20 Exercism problems
- [ ] Build 3 small Rust projects

### Month 3-4: Kernel Theory
- [ ] Read Windows Internals Ch 5, 8
- [ ] Read Filter Manager docs completely
- [ ] Watch OSR videos
- [ ] Document understanding in your own words

### Month 5-6: First Driver
- [ ] Set up development environment
- [ ] Follow windows-kernel-rs examples
- [ ] Build empty driver
- [ ] Add debug output

### Month 7-8: File Blocking
- [ ] Implement PreCreate callback
- [ ] Add path matching
- [ ] Implement blocking logic
- [ ] Test extensively

## Budget

### Free Resources (90% of what you need)
- Rust Book (Free)
- Microsoft Docs (Free)
- Community forums (Free)
- YouTube (Free)
- Tools (Free)

**Total: $0**

### Paid Resources (Nice to have)
- Windows Internals book ($60)
- Windows Kernel Programming ($40)
- Programming Rust ($50)

**Total: $150**

### Optional (If budget allows)
- OSR Seminar ($1000+) - Only if company pays
- VMware Workstation Pro ($200) - Player is free

**Total Optional: $1200**

**Recommendation**: Start with free resources. Buy books if you can afford.

## Learning Strategy

### Daily (1-2 hours)
- Read theory (30 min)
- Code practice (30-60 min)
- Document learnings (15 min)

### Weekly
- Complete 1 chapter of Rust Book
- Complete 10 Rustlings exercises
- Review and consolidate notes

### Monthly
- Reach milestone in roadmap
- Review progress
- Adjust plan if needed

## Red Flags (Bad Resources)

Avoid:
- ❌ Outdated tutorials (pre-2020)
- ❌ C-only resources without Rust perspective
- ❌ "Learn kernel programming in 24 hours"
- ❌ Tutorials that say "just copy this code"

Look for:
- ✅ Active maintenance (updated recently)
- ✅ Explains concepts, not just code
- ✅ Community-vetted (OSR, Microsoft, etc.)
- ✅ Encourages understanding over memorization

## When You're Stuck

### First 30 Minutes
1. Read error message carefully
2. Search exact error on Google
3. Check Rust docs / Microsoft docs

### After 30 Minutes
4. Search OSR forums
5. Search Rust forums

### After 2 Hours
6. Ask on Discord (Rust #beginners)
7. Post on OSR forums (with code sample)

### Never
- ❌ Give up
- ❌ Skip fundamentals
- ❌ Test on host Windows

## Progress Tracking

Create a learning journal (Markdown):

```markdown
# Week 1
## What I learned
- Rust ownership prevents memory leaks
- Borrowing rules: one mut OR many immut

## Challenges
- Struggled with lifetimes
- Solution: Drew diagrams

## Next week
- Finish Rust Book Chapter 10
- Start Chapter 11
```

## Final Advice

**Quality over quantity**
- Better to deeply understand 1 concept than superficially know 10

**Hands-on learning**
- Type every code example yourself
- Don't just copy-paste

**Teach others**
- Explain concepts to a friend
- Write blog posts
- This solidifies understanding

**Stay motivated**
- Track visible progress
- Celebrate small wins
- Remember the goal

---

**You have everything you need. Now go learn.**
