# standard python libraries
import random

class rusl():
    '''The main rusl library class.'''
    class m():
        '''The actual main class used for callable functions.'''
        
        def hw():
            '''Prints "Hello, World!"'''
            print("Hello, World!")
        
        def g(name):
            '''Greet a name in different languages.'''
            greetLanguages = ['Hello', 'Hi', 'Sup', 'Howdy', 'Greetings', 'Namaste', 'Bonjour', 'Salut', 'Hola', 'Ciao', 'Hallo', 'Hej', 'Hei', 'Merhaba', 'привет', 'Привіт', 'Γεια σας']
            greeting = random.choice(greetLanguages)
            print(greeting + ', ' + name + '! ')
        
        class i():
            '''All user interactions are in this class.'''
            
            def userInput(prompt):
                '''The `input()` function from the Python standard library, but better.<br/>
                Exports the variable `userOutput`.
                '''
                formattedPrompt = "🞂 " + prompt + " "
                
                userOutput = input(formattedPrompt)
