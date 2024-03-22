pkgdatadir = get_option('datadir') / meson.project_name()
gnome = import('gnome')

blueprints = custom_target('blueprints',
  input: files(
    'gtk/help-overlay.blp',
    'window.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

src_res = gnome.compile_resources('{{appId}}.src',
  '{{appId}}.src.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

data_res = gnome.compile_resources('{{appId}}.data',
  '{{appId}}.data.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
  dependencies: blueprints,
)

bin_conf = configuration_data()
bin_conf.set('GJS', find_program('gjs').full_path())
bin_conf.set('PACKAGE_VERSION', meson.project_version())
bin_conf.set('PACKAGE_NAME', meson.project_name())
bin_conf.set('prefix', get_option('prefix'))
bin_conf.set('libdir', get_option('prefix') / get_option('libdir'))
bin_conf.set('datadir', get_option('prefix') / get_option('datadir'))
bin_conf.set('resource_path', '/{{pathCase appId}}')

configure_file(
  input: '{{appId}}.in',
  output: '{{appId}}',
  configuration: bin_conf,
  install: true,
  install_dir: get_option('bindir')
)
