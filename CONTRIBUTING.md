# Contributing to Learn Rust Backend

Thank you for your interest in contributing to the Learn Rust repository! We welcome contributions from the community to help improve this learning resource. Below are guidelines to ensure a smooth and effective collaboration.

## How to Contribute

1. **Fork the Repository**: Click the "Fork" button on the repository page to create your own copy.

2. **Clone Your Fork**:

   ```bash
   git clone https://github.com/your-username/learn-rust-be.git
   cd learn-rust-be
   ```

3. **Create a New Branch**:
   ```bash
   git checkout -b feature/your-feature-name

4. **Install Dependencies**:
   ```bash
   pnpm install
   ```
   This will install all development dependencies including husky for commit message validation.

5. **Make Your Changes**:

- Follow the chapter structure and style guidelines.
- Ensure your contributions are clear, well-documented, and add value to the learning path.
- If adding code examples, make sure they compile and run correctly.

6. **Commit Your Changes**:

   We use conventional commits to standardize our commit messages. The format is:
   ```bash
   type(scope): subject
   ```

   Types:
   - `feat`: New features
   - `fix`: Bug fixes
   - `docs`: Documentation changes
   - `chore`: Maintenance tasks
   - `style`: Code style changes
   - `refactor`: Code refactoring
   - `ci`: CI/CD changes
   - `test`: Adding or modifying tests
   - `perf`: Performance improvements
   - `revert`: Reverting changes

   Scopes:
   - `core`: Core functionality
   - `docs`: Documentation
   - `examples`: Example code
   - `server`: Server-related
   - `db`: Database
   - `auth`: Authentication
   - `tests`: Testing
   - `config`: Configuration

   Examples:
   ```bash
   git add .
   git commit -m "feat(server): add basic HTTP server implementation"
   git commit -m "docs(core): update installation instructions"
   git commit -m "fix(auth): resolve token validation issue"
   ```

   Note: The repository is configured with commitlint and husky to enforce these conventions. Commits that don't follow this format will be rejected.

7. **Push to Your Fork:**:

   ```bash
   git push origin feature/your-feature-name
   ```

8. **Create a Pull Request**:

   - Navigate to the original learn-rust-be repository on GitHub.
   - Click "New Pull Request" and select your branch.
   - Provide a clear description of your changes and the rationale behind them.
   - Submit the pull request for review.

9. **Code Review**:
   - The maintainers will review your pull request.
   - Provide feedback and suggestions for improvement.
   - Make necessary changes based on feedback.
   - Once approved, your changes will be merged into the main repository.

## Code of Conduct
By contributing, you agree to follow our Code of Conduct. Be respectful, considerate, and collaborative. Any behavior violating this code may result in removal of contributions or banning from the repository.

## Reporting Issues
If you encounter any issues, have suggestions, or want to discuss changes:

- Open an issue on GitHub describing the problem or suggestion.
- Follow the issue template if provided, to help us understand the context better.

## Style Guidelines
- Keep contributions focused and relevant to the chapter or topic.
- Use clear and concise language.
- Format Markdown files consistently, following standard Markdown conventions.

## Licensing
By contributing, you agree that your contributions will be licensed under the MIT License, as specified in the LICENSE file.

Thank you for helping to improve the Learn Rust repository!

<p align="center"> <sub>© 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="./LICENSE">MIT License</a>.</sub> </p>