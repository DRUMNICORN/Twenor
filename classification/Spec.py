# -*- mode: python ; coding: utf-8 -*-
from PyInstaller.utils.hooks import collect_data_files

missing_packages = ['librosa', 'tensorflow', 'keras', 'sklearn']

hiddenimports = ['ffmpeg-python', 'sklearn.neighbors.typedefs', 'sklearn.utils._cython_blas','sklearn.neighbors._typedefs', 'sklearn.neighbors._quad_tree', 'sklearn.tree._utils', 'sklearn.utils._weight_vector', 'sklearn.tree', 'cython']
hiddenimports += missing_packages

datas = []
for package in missing_packages:
    datas += collect_data_files(package)

datas += [('img/*.png','img/')]
datas += [('img/*.jpg','img/')]
datas += [('styles/*.css','styles/')]

block_cipher = None

a = Analysis(
    ['App.py'],
    pathex=[],
    binaries=[],
    datas=datas,
    hiddenimports=hiddenimports,
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)
pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='App',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True, # TODO deactivate CONSOLE
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon='img\\logo.ico',
)