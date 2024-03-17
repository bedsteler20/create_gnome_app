gnome = import('gnome')

blueprints = custom_target('blueprints',
  input: files(
    'gtk/help-overlay.blp',
    '{{dashCase projectName}}-window.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

{{snakeCase projectName}}_sources = [
  'main.c',
  '{{dashCase projectName}}-application.c',
  '{{dashCase projectName}}-window.c',
]

{{snakeCase projectName}}_sources += gnome.compile_resources('{{dashCase projectName}}-resources',
  '{{dashCase projectName}}.gresource.xml',
  c_name: '{{snakeCase projectName}}',
  dependencies: blueprints,
)


{{snakeCase projectName}}_deps = [
  dependency('gtk4'),
  dependency('libadwaita-1', version: '>= 1.4'),
]


executable('{{dashCase projectName}}', {{snakeCase projectName}}_sources,
  dependencies: {{snakeCase projectName}}_deps,
       install: true,
)
