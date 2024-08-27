from src.core.Base import Base

import pickle
import joblib

import os
# import IPython
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
sns.set_style('whitegrid')
# %matplotlib inline

import tensorflow as tf
print("TF version:-", tf.__version__)
import keras as k


import warnings
warnings.filterwarnings('ignore')
import sklearn.model_selection as skms
import sklearn.preprocessing as skp

seed = 42
tf.random.set_seed(seed)
np.random.seed(seed) 

class Computerization(Base):
    def preperation(self):
        self.df.drop(['length'], axis=1, inplace=True)
        
        self.encodeGenreLabel()
        self.df.label = [self.label_index[l] for l in self.df.label]
        
        self.splitSets()
        self.fitSets()
        self.models = []
        self.historys = []
        
    def splitSets(self):
        self.df = self.df.sample(frac=1, random_state=seed).reset_index(drop=True) 
    
        # remove irrelevant columns
        self.df.drop(['filename'], axis=1, inplace=True)
        self.df = self.df.reindex(sorted(self.df.columns), axis=1)
        
        df_y = self.df.pop('label')
        df_X = self.df
        
        # split into train dev and test
        self.X_train, self.X_dev, self.y_train, self.y_dev = skms.train_test_split(df_X, df_y, train_size=0.8, random_state=seed, stratify=df_y)
    
    def fitSets(self):
        self.scaler = skp.StandardScaler()
        self.X_train = pd.DataFrame(self.scaler.fit_transform(self.X_train), columns=self.X_train.columns)
        self.X_dev = pd.DataFrame(self.scaler.transform(self.X_dev), columns=self.X_train.columns)

    class myCallback(k.callbacks.Callback):
        ACCURACY_THRESHOLD = 0.98
        def on_epoch_end(self, epoch, logs={}):
            if(logs.get('val_accuracy') > self.ACCURACY_THRESHOLD):
                print("\n\nStopping training as we have reached %2.2f%% accuracy!" %(self.ACCURACY_THRESHOLD*100))   
                self.model.stop_training = True
                
    def trainModel(self, model, epochs, optimizer):
        # Stop training when a monitored metric has stopped improving.
        callbacks = [   tf.keras.callbacks.EarlyStopping(monitor='loss', patience=3),
                        self.myCallback() ]
        
        model.compile(optimizer=optimizer,
                    loss='sparse_categorical_crossentropy',
                    metrics='accuracy' )
        
        print('STRUCT', self.X_train.columns, len(self.X_train.columns))
        return model.fit(self.X_train, self.y_train, validation_data=(self.X_dev, self.y_dev), epochs=epochs, 
                        batch_size=self.batch_size, callbacks=callbacks)


    def buildModelA(self):
        model = k.models.Sequential([
            k.layers.Dense(256, activation='relu', input_shape=(self.X_train.shape[1],)),
            k.layers.Dense(128, activation='relu'),
            k.layers.Dense(64, activation='relu'),
            k.layers.Dense(len(self.label_index.keys()), activation='softmax'),
        ])
        self.build(model, 70, 'adam') 

    def buildModelB(self):
        model = k.models.Sequential([
            k.layers.Dense(512, activation='relu', input_shape=(self.X_train.shape[1],)),
            k.layers.Dropout(0.2),

            k.layers.Dense(256, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(128, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(64, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(len(self.label_index.keys()), activation='softmax'),
        ])
        self.build(model, 100, 'adam') 

    def buildModelC(self):
        model = k.models.Sequential([
            k.layers.Dense(512, activation='relu', input_shape=(self.X_train.shape[1],)),
            k.layers.Dropout(0.2),

            k.layers.Dense(256, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(128, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(64, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(len(self.label_index.keys()), activation='softmax'),
        ])
        self.build(model, 700, 'sgd') 

    def buildModelD(self):
        model = k.models.Sequential([
            k.layers.Dense(1024, activation='relu', input_shape=(self.X_train.shape[1],)),
            k.layers.Dropout(0.3),

            k.layers.Dense(512, activation='relu'),
            k.layers.Dropout(0.3),

            k.layers.Dense(256, activation='relu'),
            k.layers.Dropout(0.3),

            k.layers.Dense(128, activation='relu'),
            k.layers.Dropout(0.3),

            k.layers.Dense(64, activation='relu'),
            k.layers.Dropout(0.3),

            k.layers.Dense(len(self.label_index.keys()), activation='softmax'),
        ])
        self.build(model, 500, 'rmsprop') 

    def buildModelE(self):
        model = k.models.Sequential([
            k.layers.Dense(512, activation='relu', input_shape=(self.X_train.shape[1],)),
            k.layers.Dropout(0.2),

            k.layers.Dense(256, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(128, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(64, activation='relu'),
            k.layers.Dropout(0.2),

            k.layers.Dense(len(self.label_index.keys()), activation='softmax'),
        ])
        self.build(model, 100, 'adam') 

    def build(self, model, epochs, optimizer):
        print(model.summary())
        history = self.trainModel(model=model, epochs=epochs, optimizer=optimizer)
        self.models = np.append(self.models, model)
        self.historys = np.append(self.historys, history) 

    def generateHistory(self):
        for i, history in enumerate(self.historys):
            print(f"Model {i}: Max. Validation Accuracy is {max(history.history['val_accuracy'])}%")
            pd.DataFrame(history.history).plot(figsize=(12,6))
            plt.show()
            # plt.draw()
            # plt.savefig(os.path.join(self.data_dir, f"History{i}_ChartPlot.png"))
          
    def encodeGenreLabel(self):
        self.label_index = {}
        self.index_label = {}
        for i, x in enumerate(self.df.label.unique()):
            self.label_index[x] = i
            self.index_label[i] = x
        
    def getBestModelIndex(self):
        best_model_index = 0
        best_acc = 0
        for i, history in enumerate(self.historys):
            acc = max(history.history["val_accuracy"])
            if (acc > best_acc):
                best_acc = acc
                best_model_index = i
        print(f"best model is {best_model_index}")
        return best_model_index
        
    def save(self):
        
        self.saveModels()
        
        joblib.dump(self.scaler , os.path.join(self.data_dir, "scaler.pkl"))     # save to disk

        with open(os.path.join(self.data_dir, "columns.pkl"), "wb") as a_file:
            pickle.dump(self.X_train.columns, a_file)

        with open(os.path.join(self.data_dir, "labels.pkl"), "wb") as a_file:
            pickle.dump(self.index_label, a_file)
        
    def saveModels(self):
        best_model_index = self.getBestModelIndex()

        for i, history in enumerate(self.historys):
            acc = max(history.history["val_accuracy"])
            self.models[i].save(os.path.join(self.data_dir, f'model[{i}]_{round(acc*100)/100}'+ ('_best' if i == best_model_index else '') + '.h5'))
