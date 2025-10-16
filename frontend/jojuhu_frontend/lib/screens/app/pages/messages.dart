import 'package:flutter/material.dart';

class Message {
  String text;
  int isSent;

  Message(this.text, this.isSent);
}

class MessagesScreen extends StatefulWidget {
  const MessagesScreen({super.key});

  @override
  State<MessagesScreen> createState() => _MessagesScreen();
}

class _MessagesScreen extends State<MessagesScreen> {
  int currentPageIndex = 0;

  @override
  Widget build(BuildContext context) {
    final ThemeData theme = Theme.of(context);
    List<Message> messages = [
      Message("Opa", 1),
      Message("Dia, de boa?", 2),
      Message("Bao e vc?", 1)
    ];

    return Scaffold(
      body: ListView.builder(
        reverse: false,
        itemCount: messages.length,
        itemBuilder: (BuildContext context, int index) {
          Message message = messages[index];
          return Align(
            alignment: message.isSent == 1
                ? Alignment.centerLeft
                : Alignment.centerRight,
            child: Container(
              margin: const EdgeInsets.all(8.0),
              padding: const EdgeInsets.all(8.0),
              decoration: BoxDecoration(
                color: theme.colorScheme.primary,
                borderRadius: BorderRadius.circular(8.0),
              ),
              child: Text(
                message.text,
                style: theme.textTheme.bodyLarge!
                    .copyWith(color: theme.colorScheme.onPrimary),
              ),
            ),
          );
        },
      ),
    );
  }
}
