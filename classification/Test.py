import sys
from PySide2.QtWidgets import QApplication, QDialog, QLineEdit, QPushButton, QVBoxLayout

class Form(QDialog):

    def __init__(self, parent=None):
        super(Form, self).__init__(parent)
        self.setWindowTitle("My Form")
        
        # Create layout and add widgets
        layout = QVBoxLayout()
        layout.addWidget(QPushButton('lol'))
        # Set dialog layout
        self.setLayout(layout)


if __name__ == '__main__':
    # Create the Qt Application
    app = QApplication(sys.argv)
    # Create and show the form
    form = Form()
    form.show()
    

    
    # Run the main Qt loop
    sys.exit(app.exec_())