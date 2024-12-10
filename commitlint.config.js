module.exports = {
    extends: ['@commitlint/config-conventional'],
    rules: {
      'type-enum': [
        2,
        'always',
        [
          'feat',    // New features
          'fix',     // Bug fixes
          'docs',    // Documentation changes
          'chore',   // Maintenance tasks
          'style',   // Code style changes
          'refactor',// Code refactoring
          'ci',      // CI/CD changes
          'test',    // Adding or modifying tests
          'perf',    // Performance improvements
          'revert'   // Reverting changes
        ],
      ],
      'scope-enum': [
        2,
        'always',
        [
          'core',
          'docs',
          'examples',
          'server',
          'db',
          'auth',
          'tests',
          'config'
        ]
      ]
    }
  };