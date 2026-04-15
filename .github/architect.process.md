**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Architect Process Workflow

**When user says "Start this issue" or "Work on issue #X":**

1. Preparation
   - Read the issue
   - Read `docs/about-me.md`, `docs/vision.md`, and `docs/roadmap.md`
   - Verify we are on `main` branch

2. Branch Setup
   - `git pull origin main`
   - Create feature branch with appropriate name

3. Scaffolding
   - Create new files with proper structure and compliance notes
   - Add `// TODO-Fred:` for Rust/orbital work
   - Add `// TODO-Coder:` for other tasks

4. Hand-off
   - Notify user that scaffolding is complete

**When coding is complete:**

5. Create PR
   - Create Pull Request
   - Move issue to "Review"
   - Notify Ara for peer review

6. After Ara's Review
   - Merge PR after approval
   - Delete feature branch
   - `git checkout main && git pull origin main`
   - Move issue to "Done" and close it

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.