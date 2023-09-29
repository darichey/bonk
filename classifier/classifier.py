import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import Dataset, DataLoader
import pandas as pd
import string

class CNN(nn.Module):
    def __init__(self, num_categories):
        super(CNN, self).__init__()
        self.conv1 = nn.Conv1d(in_channels=256, out_channels=16, kernel_size=7)
        self.fc = nn.Linear(in_features=1952, out_features=num_categories)
    
    def forward(self, x):
        x = self.conv1(x)
        x = x.view(x.size(0), -1)
        x = self.fc(x)
        return x

def one_hot_encode(input_string):
    encoded_tensor = torch.zeros(len(input_string), 128)
    
    for i, char in enumerate(input_string):
        encoded_tensor[i, ord(char)] = 1
    
    return encoded_tensor

def remove_non_ascii(input_string):
    return ''.join(char if char in string.printable else ' ' for char in input_string).ljust(256)[:256]

def transaction_to_input(transaction):
    return one_hot_encode(remove_non_ascii(transaction).lower())
    

class TransactionDataset(Dataset):
    def __init__(self, dataframe, category_to_label):
        self.data = dataframe
        self.category_to_label = category_to_label

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        transaction = transaction_to_input(self.data.iloc[idx]['transaction'])
        category = self.category_to_label[self.data.iloc[idx]['category']]
        return transaction, category


def get_data_loaders():
    df = pd.read_csv('transaction_data.csv')
    category_to_label = {category: label for label, category in enumerate(df['category'].unique())}
    label_to_category = {label: category for category, label in category_to_label.items()}

    train_df = df.sample(frac=0.8)
    test_df = df.drop(train_df.index)

    train_dataset = TransactionDataset(train_df, category_to_label)
    test_dataset = TransactionDataset(test_df, category_to_label)

    batch_size = 1
    train_loader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True)
    test_loader = DataLoader(test_dataset, batch_size=batch_size, shuffle=False)

    return train_loader, test_loader, df['category'].nunique(), label_to_category

def train(train_loader, num_categories):
    cnn = CNN(num_categories)
    criterion = nn.CrossEntropyLoss()
    optimizer = optim.Adam(cnn.parameters(), lr=0.001)

    num_epochs = 10
    for epoch in range(num_epochs):
        print(epoch)
        for inputs, labels in train_loader:
            optimizer.zero_grad()
            outputs = cnn(inputs)
            loss = criterion(outputs, labels)
            loss.backward()
            optimizer.step()
    
    return cnn

def test_accuracy(cnn, test_loader):
    correct = 0
    total = 0
    with torch.no_grad():
        for inputs, labels in test_loader:
            outputs = cnn(inputs)
            _, predicted = torch.max(outputs.data, 1)
            total += labels.size(0)
            correct += (predicted == labels).sum().item()

    accuracy = 100 * correct / total
    print(f'Test Accuracy: {accuracy:.2f}%')


(train_loader, test_loader, num_categories, label_to_category) = get_data_loaders()
cnn = train(train_loader, num_categories)
cnn.eval()
test_accuracy(cnn, test_loader)

while True:
    transaction = input()
    inputs = transaction_to_input(transaction).unsqueeze(0)
    outputs = cnn(inputs)
    _, predicted = torch.max(outputs.data, 1)
    print(label_to_category[predicted.item()])