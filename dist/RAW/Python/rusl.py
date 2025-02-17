# standard python libraries
import random
import os

class rusl():
    '''The main rusl library class.'''

    ruslver = "0.1.2\n"

    def v(ruslver):
        '''Prints the current used rusl version.'''
        print("rusl" + " - " + ruslver)

    NL = "\n"

    def cls(printversion="no"):
        """
        Clears the console and optionally prints the version of rusl.

        Parameters:
            printversion (str): A string that should be 'yes' or 'no'.
                                If 'yes' (or 'y'), the version is printed.
                                If 'no' (or 'n'), only the console is cleared.

        Raises:
            ValueError: If printversion is not 'yes' or 'no'.
            TypeError: If printversion is not a string.
        """
        os.system('cls' if os.name == 'nt' else 'clear')

        if not isinstance(printversion, str):
            raise TypeError("printversion must be a string ('yes' or 'no').")

        response = printversion.strip().lower()

        if response in ('yes', 'y'):
            rusl.v()
        elif response in ('no', 'n'):
            # Do nothing extra if the user does not want to print the version.
            pass
        else:
            raise ValueError("Invalid argument for rusl.cls(printversion). Use 'yes' or 'no'.")

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
            '''The `input()` function from the Python standard library, but better.

            Returns the variable `userOutput`.
            '''
            formattedPrompt = "🞂 | " + prompt + " "
            userOutput = input(formattedPrompt)
            return userOutput
