# Contributing to Nextgen-cli 🚀

First off, thank you for considering contributing to Nextgen-cli! It's people like you that make this tool better for the GitDigital ecosystem.

## 🛠 Local Development Setup

1. **Fork and Clone**:
   ```bash
   git clone [https://github.com/YOUR_USERNAME/Nextgen-cli.git](https://github.com/YOUR_USERNAME/Nextgen-cli.git)
   cd Nextgen-cli
```

To turn Nextgen-cli into a high-standard, professional repository, here are the three essential components we discussed. You can copy these directly into your project.
1. CONTRIBUTING.md
Place this in your root directory. It sets the ground rules for how others should submit code.
# Contributing to Nextgen-cli 🚀

First off, thank you for considering contributing to Nextgen-cli! It's people like you that make this tool better for the GitDigital ecosystem.

## 🛠 Local Development Setup

1. **Fork and Clone**:
   ```bash
   git clone [https://github.com/YOUR_USERNAME/Nextgen-cli.git](https://github.com/YOUR_USERNAME/Nextgen-cli.git)
   cd Nextgen-cli

 * Install Dependencies:
   npm install

 * Build & Link:
   npm run build
npm link

🧪 Running Tests
Please ensure all tests pass before submitting a PR:
npm test

📮 Pull Request Process
 * Create a new branch: git checkout -b feature/your-feature-name.
 * Commit your changes with descriptive messages.
 * Push to your fork and submit a Pull Request.
 * Ensure your PR is linked to an existing Issue.
By contributing, you agree that your code will be licensed under the project's MIT License.

---

### 2. GitHub Action: Auto-Publish to NPM
Create a file at `.github/workflows/publish.yml`. This will automatically publish your CLI to NPM whenever you push a tag (like `v1.0.1`) and includes **Provenance** (a security gold standard that proves the code came from your GitHub repo).

```yaml
name: Publish to NPM

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      id-token: write # Required for provenance
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions/setup-node@v4
        with:
          node-version: '20.x'
          registry-url: 'https://registry.npmjs.org'
          
      - run: npm ci
      - run: npm test
      - run: npm run build

      - name: Publish to NPM
        run: npm publish --provenance --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}

> Note: You will need to add your NPM_TOKEN to Settings > Secrets and variables > Actions in your GitHub repo.
> 
3. Issue Templates (The "Professional" Look)
Create a folder named .github/ISSUE_TEMPLATE/ and add these two files. This adds a "New Issue" selector for your users.
bug_report.md:
---
name: "🐛 Bug Report"
about: Create a report to help us improve.
labels: bug
---

**Describe the bug**
A clear and concise description of what the bug is.

**Steps to Reproduce**
1. Run command 'nextgen-cli ...'
2. See error

**Expected behavior**
What you expected to happen.

**Environment:**
- OS: [e.g. macOS, Ubuntu]
- Node Version: [e.g. 20.10.0]

feature_request.md:
---
name: "✨ Feature Request"
about: Suggest an idea for this project.
labels: enhancement
---

**Is your feature request related to a problem?**
A clear description of what the problem is.

**Describe the solution you'd like**
A clear description of what you want to happen.