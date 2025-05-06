#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QLibrary>
#include <QMessageBox>
#include <cmath>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
    , trigonometryLib(nullptr)  // Inisialisasi pointer
{
    ui->setupUi(this);
    loadTrigonometryLibrary();
}

MainWindow::~MainWindow()
{
    unloadTrigonometryLibrary();
    delete ui;
}

void MainWindow::loadTrigonometryLibrary()
{
    QString libraryPath = QCoreApplication::applicationDirPath() + "/libtrigonometry.so";
    trigonometryLib = new QLibrary(libraryPath);
    
    if (!trigonometryLib->load()) {
        QMessageBox::warning(this, "Error", "Failed to load trigonometry library: " + trigonometryLib->errorString());
        delete trigonometryLib;
        trigonometryLib = nullptr;
        return;
    }

    init = (void(*)())trigonometryLib->resolve("init");
    calculate = (void(*)(double, int, double*, double*, double*, double*, double*, double*))trigonometryLib->resolve("calculate_trigonometry");

    if (!init || !calculate) {
        QMessageBox::warning(this, "Error", "Failed to resolve library functions");
        unloadTrigonometryLibrary();
        return;
    }

    init(); // Initialize the library
}

void MainWindow::unloadTrigonometryLibrary()
{
    if (trigonometryLib) {
        trigonometryLib->unload();
        delete trigonometryLib;
        trigonometryLib = nullptr;
    }
}

void MainWindow::on_calculateButton_clicked()
{
    bool ok;
    double angle = ui->angleInput->text().toDouble(&ok);
    if (!ok) {
        QMessageBox::warning(this, "Invalid Input", "Please enter a valid angle");
        return;
    }

    int terms = ui->termsInput->text().toInt(&ok);
    if (!ok || terms <= 0) {
        terms = 10; // Default value
    }

    calculateTrigonometry(angle, terms);
}

void MainWindow::calculateTrigonometry(double angle, int terms)
{
    if (!calculate) {
        QMessageBox::warning(this, "Error", "Library not loaded properly");
        return;
    }

    double sin_t, cos_t, sin_lut, cos_lut, sin_std, cos_std;
    calculate(angle, terms, &sin_t, &cos_t, &sin_lut, &cos_lut, &sin_std, &cos_std);

    // Update UI with results
    ui->taylorSinResult->setText(QString::number(sin_t, 'g', 15));
    ui->taylorCosResult->setText(QString::number(cos_t, 'g', 15));
    
    if (std::isnan(sin_lut)) {
        ui->lookupSinResult->setText("N/A (non-integer angle)");
        ui->lookupCosResult->setText("N/A (non-integer angle)");
    } else {
        ui->lookupSinResult->setText(QString::number(sin_lut, 'g', 15));
        ui->lookupCosResult->setText(QString::number(cos_lut, 'g', 15));
    }
    
    ui->stdlibSinResult->setText(QString::number(sin_std, 'g', 15));
    ui->stdlibCosResult->setText(QString::number(cos_std, 'g', 15));
}