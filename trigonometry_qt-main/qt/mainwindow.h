#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QLibrary>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void on_calculateButton_clicked();

private:
    Ui::MainWindow *ui;
    void loadTrigonometryLibrary();
    void unloadTrigonometryLibrary();
    void calculateTrigonometry(double angle, int terms);
    
    // Function pointers for Rust library
    void (*init)();
    void (*calculate)(double, int, double*, double*, double*, double*, double*, double*);
    QLibrary* trigonometryLib;  // Menggunakan pointer QLibrary
};

#endif // MAINWINDOW_H