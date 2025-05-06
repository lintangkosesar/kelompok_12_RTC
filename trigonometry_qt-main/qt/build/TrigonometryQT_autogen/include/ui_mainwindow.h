/********************************************************************************
** Form generated from reading UI file 'mainwindow.ui'
**
** Created by: Qt User Interface Compiler version 5.15.3
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_MAINWINDOW_H
#define UI_MAINWINDOW_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QFormLayout>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_MainWindow
{
public:
    QWidget *centralwidget;
    QVBoxLayout *verticalLayout;
    QLabel *logoLabel;
    QFormLayout *formLayout;
    QLabel *label;
    QLineEdit *angleInput;
    QLabel *label_2;
    QLineEdit *termsInput;
    QPushButton *calculateButton;
    QGroupBox *groupBox;
    QFormLayout *formLayout_2;
    QLabel *label_3;
    QLabel *taylorSinResult;
    QLabel *label_4;
    QLabel *taylorCosResult;
    QLabel *label_5;
    QLabel *lookupSinResult;
    QLabel *label_6;
    QLabel *lookupCosResult;
    QLabel *label_7;
    QLabel *stdlibSinResult;
    QLabel *label_8;
    QLabel *stdlibCosResult;

    void setupUi(QMainWindow *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName(QString::fromUtf8("MainWindow"));
        MainWindow->resize(600, 400);
        centralwidget = new QWidget(MainWindow);
        centralwidget->setObjectName(QString::fromUtf8("centralwidget"));
        verticalLayout = new QVBoxLayout(centralwidget);
        verticalLayout->setObjectName(QString::fromUtf8("verticalLayout"));
        logoLabel = new QLabel(centralwidget);
        logoLabel->setObjectName(QString::fromUtf8("logoLabel"));
        logoLabel->setPixmap(QPixmap(QString::fromUtf8(":/images/logo.png")));
        logoLabel->setAlignment(Qt::AlignCenter);

        verticalLayout->addWidget(logoLabel);

        formLayout = new QFormLayout();
        formLayout->setObjectName(QString::fromUtf8("formLayout"));
        label = new QLabel(centralwidget);
        label->setObjectName(QString::fromUtf8("label"));

        formLayout->setWidget(0, QFormLayout::LabelRole, label);

        angleInput = new QLineEdit(centralwidget);
        angleInput->setObjectName(QString::fromUtf8("angleInput"));

        formLayout->setWidget(0, QFormLayout::FieldRole, angleInput);

        label_2 = new QLabel(centralwidget);
        label_2->setObjectName(QString::fromUtf8("label_2"));

        formLayout->setWidget(1, QFormLayout::LabelRole, label_2);

        termsInput = new QLineEdit(centralwidget);
        termsInput->setObjectName(QString::fromUtf8("termsInput"));

        formLayout->setWidget(1, QFormLayout::FieldRole, termsInput);


        verticalLayout->addLayout(formLayout);

        calculateButton = new QPushButton(centralwidget);
        calculateButton->setObjectName(QString::fromUtf8("calculateButton"));

        verticalLayout->addWidget(calculateButton);

        groupBox = new QGroupBox(centralwidget);
        groupBox->setObjectName(QString::fromUtf8("groupBox"));
        formLayout_2 = new QFormLayout(groupBox);
        formLayout_2->setObjectName(QString::fromUtf8("formLayout_2"));
        label_3 = new QLabel(groupBox);
        label_3->setObjectName(QString::fromUtf8("label_3"));

        formLayout_2->setWidget(0, QFormLayout::LabelRole, label_3);

        taylorSinResult = new QLabel(groupBox);
        taylorSinResult->setObjectName(QString::fromUtf8("taylorSinResult"));

        formLayout_2->setWidget(0, QFormLayout::FieldRole, taylorSinResult);

        label_4 = new QLabel(groupBox);
        label_4->setObjectName(QString::fromUtf8("label_4"));

        formLayout_2->setWidget(1, QFormLayout::LabelRole, label_4);

        taylorCosResult = new QLabel(groupBox);
        taylorCosResult->setObjectName(QString::fromUtf8("taylorCosResult"));

        formLayout_2->setWidget(1, QFormLayout::FieldRole, taylorCosResult);

        label_5 = new QLabel(groupBox);
        label_5->setObjectName(QString::fromUtf8("label_5"));

        formLayout_2->setWidget(2, QFormLayout::LabelRole, label_5);

        lookupSinResult = new QLabel(groupBox);
        lookupSinResult->setObjectName(QString::fromUtf8("lookupSinResult"));

        formLayout_2->setWidget(2, QFormLayout::FieldRole, lookupSinResult);

        label_6 = new QLabel(groupBox);
        label_6->setObjectName(QString::fromUtf8("label_6"));

        formLayout_2->setWidget(3, QFormLayout::LabelRole, label_6);

        lookupCosResult = new QLabel(groupBox);
        lookupCosResult->setObjectName(QString::fromUtf8("lookupCosResult"));

        formLayout_2->setWidget(3, QFormLayout::FieldRole, lookupCosResult);

        label_7 = new QLabel(groupBox);
        label_7->setObjectName(QString::fromUtf8("label_7"));

        formLayout_2->setWidget(4, QFormLayout::LabelRole, label_7);

        stdlibSinResult = new QLabel(groupBox);
        stdlibSinResult->setObjectName(QString::fromUtf8("stdlibSinResult"));

        formLayout_2->setWidget(4, QFormLayout::FieldRole, stdlibSinResult);

        label_8 = new QLabel(groupBox);
        label_8->setObjectName(QString::fromUtf8("label_8"));

        formLayout_2->setWidget(5, QFormLayout::LabelRole, label_8);

        stdlibCosResult = new QLabel(groupBox);
        stdlibCosResult->setObjectName(QString::fromUtf8("stdlibCosResult"));

        formLayout_2->setWidget(5, QFormLayout::FieldRole, stdlibCosResult);


        verticalLayout->addWidget(groupBox);

        MainWindow->setCentralWidget(centralwidget);

        retranslateUi(MainWindow);

        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QMainWindow *MainWindow)
    {
        MainWindow->setWindowTitle(QCoreApplication::translate("MainWindow", "Trigonometry Calculator", nullptr));
        label->setText(QCoreApplication::translate("MainWindow", "Angle (degrees):", nullptr));
        label_2->setText(QCoreApplication::translate("MainWindow", "Taylor Terms:", nullptr));
        termsInput->setText(QCoreApplication::translate("MainWindow", "10", nullptr));
        calculateButton->setText(QCoreApplication::translate("MainWindow", "Calculate", nullptr));
        groupBox->setTitle(QCoreApplication::translate("MainWindow", "Results", nullptr));
        label_3->setText(QCoreApplication::translate("MainWindow", "Taylor Sin:", nullptr));
        taylorSinResult->setText(QString());
        label_4->setText(QCoreApplication::translate("MainWindow", "Taylor Cos:", nullptr));
        taylorCosResult->setText(QString());
        label_5->setText(QCoreApplication::translate("MainWindow", "Lookup Sin:", nullptr));
        lookupSinResult->setText(QString());
        label_6->setText(QCoreApplication::translate("MainWindow", "Lookup Cos:", nullptr));
        lookupCosResult->setText(QString());
        label_7->setText(QCoreApplication::translate("MainWindow", "Stdlib Sin:", nullptr));
        stdlibSinResult->setText(QString());
        label_8->setText(QCoreApplication::translate("MainWindow", "Stdlib Cos:", nullptr));
        stdlibCosResult->setText(QString());
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_MAINWINDOW_H
