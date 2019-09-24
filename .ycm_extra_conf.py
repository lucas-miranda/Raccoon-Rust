def Settings( **kwargs ):
    if kwargs['language'] == 'rust':
        return { 
            'ls': {
                'features': [ 'dx12' ]
            }
        }
