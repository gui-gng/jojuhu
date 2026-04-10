import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import '../theme/jojuhu_theme.dart';
import '../theme/tokens.dart';

/// Jojuhu Text Field Widget
/// Standardized input field with validation states and focus animations

enum JojuhuTextFieldVariant { outline, filled, underline }

class JojuhuTextField extends StatefulWidget {
  final String? label;
  final String? hint;
  final String? errorText;
  final String? helperText;
  final IconData? prefixIcon;
  final IconData? suffixIcon;
  final VoidCallback? onSuffixTap;
  final bool obscureText;
  final bool enabled;
  final bool readOnly;
  final int maxLines;
  final int? maxLength;
  final TextInputType? keyboardType;
  final TextEditingController? controller;
  final ValueChanged<String>? onChanged;
  final VoidCallback? onTap;
  final FormFieldValidator<String>? validator;
  final FocusNode? focusNode;
  final TextInputAction? textInputAction;
  final ValueChanged<String>? onSubmitted;
  final JojuhuTextFieldVariant variant;

  const JojuhuTextField({
    super.key,
    this.label,
    this.hint,
    this.errorText,
    this.helperText,
    this.prefixIcon,
    this.suffixIcon,
    this.onSuffixTap,
    this.obscureText = false,
    this.enabled = true,
    this.readOnly = false,
    this.maxLines = 1,
    this.maxLength,
    this.keyboardType,
    this.controller,
    this.onChanged,
    this.onTap,
    this.validator,
    this.focusNode,
    this.textInputAction,
    this.onSubmitted,
    this.variant = JojuhuTextFieldVariant.outline,
    this.autofocus = false,
  });

  final bool autofocus;

  @override
  State<JojuhuTextField> createState() => _JojuhuTextFieldState();
}

class _JojuhuTextFieldState extends State<JojuhuTextField> {
  late FocusNode _internalFocusNode;
  bool _isFocused = false;

  @override
  void initState() {
    super.initState();
    _internalFocusNode = widget.focusNode ?? FocusNode();
    _internalFocusNode.addListener(_onFocusChange);
  }

  @override
  void dispose() {
    _internalFocusNode.removeListener(_onFocusChange);
    if (widget.focusNode == null) {
      _internalFocusNode.dispose();
    }
    super.dispose();
  }

  void _onFocusChange() {
    setState(() {
      _isFocused = _internalFocusNode.hasFocus;
    });
  }

  @override
  Widget build(BuildContext context) {
    Color borderColor = JojuhuColors.border;
    if (widget.errorText != null) {
      borderColor = JojuhuColors.error;
    } else if (_isFocused) {
      borderColor = JojuhuColors.sol;
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.label != null) ...[Text(
            widget.label!,
            style: Theme.of(context).textTheme.labelLarge,
          ),
          SizedBox(height: JojuhuSpacing.xs.h),
        ],
        AnimatedContainer(
          duration: JojuhuDuration.quick,
          decoration: BoxDecoration(
            borderRadius: JojuhuRadius.smRadius,
            boxShadow: _isFocused
                ? [BoxShadow(
                    color: JojuhuColors.sol.withOpacity(0.1),
                    blurRadius: 8,
                    spreadRadius: 2,
                  )]
                : [],
          ),
          child: TextFormField(
            controller: widget.controller,
            focusNode: _internalFocusNode,
            autofocus: widget.autofocus,
            obscureText: widget.obscureText,
            enabled: widget.enabled,
            readOnly: widget.readOnly,
            maxLines: widget.maxLines,
            maxLength: widget.maxLength,
            keyboardType: widget.keyboardType,
            textInputAction: widget.textInputAction,
            onChanged: widget.onChanged,
            onTap: widget.onTap,
            onFieldSubmitted: widget.onSubmitted,
            validator: widget.validator,
            style: Theme.of(context).textTheme.bodyLarge,
            decoration: InputDecoration(
              hintText: widget.hint,
              errorText: widget.errorText,
              helperText: widget.helperText,
              prefixIcon: widget.prefixIcon != null
                  ? Icon(widget.prefixIcon,
                      color: _isFocused
                          ? JojuhuColors.sol
                          : JojuhuColors.textoCinza,
                      size: JojuhuInsets.iconMd.r,
                    )
                  : null,
              suffixIcon: widget.suffixIcon != null
                  ? IconButton(
                      icon: Icon(
                        widget.suffixIcon,
                        color: JojuhuColors.textoCinza,
                        size: JojuhuInsets.iconMd.r,
                      ),
                      onPressed: widget.onSuffixTap,
                    )
                  : null,
              filled: widget.variant == JojuhuTextFieldVariant.filled,
              fillColor: widget.variant == JojuhuTextFieldVariant.filled
                  ? JojuhuColors.fundo
                  : Colors.transparent,
              border: _getBorder(widget.variant),
              enabledBorder: _getBorder(widget.variant),
              focusedBorder: _getFocusedBorder(widget.variant),
              errorBorder: _getErrorBorder(widget.variant),
              focusedErrorBorder: _getFocusedErrorBorder(widget.variant),
              contentPadding: EdgeInsets.symmetric(
                horizontal: JojuhuSpacing.lg.w,
                vertical: JojuhuSpacing.md.h,
              ),
            ),
          ),
        ),
      ],
    );
  }

  InputBorder _getBorder(JojuhuTextFieldVariant variant) {
    switch (variant) {
      case JojuhuTextFieldVariant.outline:
        return OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.border),
        );
      case JojuhuTextFieldVariant.filled:
        return OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: BorderSide.none,
        );
      case JojuhuTextFieldVariant.underline:
        return const UnderlineInputBorder(
          borderSide: BorderSide(color: JojuhuColors.border),
        );
    }
  }

  InputBorder _getFocusedBorder(JojuhuTextFieldVariant variant) {
    switch (variant) {
      case JojuhuTextFieldVariant.outline:
        return OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.sol, width: 2),
        );
      case JojuhuTextFieldVariant.filled:
        return OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.sol, width: 2),
        );
      case JojuhuTextFieldVariant.underline:
        return const UnderlineInputBorder(
          borderSide: BorderSide(color: JojuhuColors.sol, width: 2),
        );
    }
  }

  InputBorder _getErrorBorder(JojuhuTextFieldVariant variant) {
    switch (variant) {
      case JojuhuTextFieldVariant.outline:
      case JojuhuTextFieldVariant.filled:
        return OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.error),
        );
      case JojuhuTextFieldVariant.underline:
        return const UnderlineInputBorder(
          borderSide: BorderSide(color: JojuhuColors.error),
        );
    }
  }

  InputBorder _getFocusedErrorBorder(JojuhuTextFieldVariant variant) {
    switch (variant) {
      case JojuhuTextFieldVariant.outline:
      case JojuhuTextFieldVariant.filled:
        return OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.error, width: 2),
        );
      case JojuhuTextFieldVariant.underline:
        return const UnderlineInputBorder(
          borderSide: BorderSide(color: JojuhuColors.error, width: 2),
        );
    }
  }
}

/// Search input variant
class JojuhuSearchField extends StatelessWidget {
  final String? hint;
  final ValueChanged<String>? onChanged;
  final VoidCallback? onTap;
  final TextEditingController? controller;
  final bool autofocus;

  const JojuhuSearchField({
    super.key,
    this.hint = 'Search...',
    this.onChanged,
    this.onTap,
    this.controller,
    this.autofocus = false,
  });

  @override
  Widget build(BuildContext context) {
    return JojuhuTextField(
      hint: hint,
      prefixIcon: Icons.search,
      controller: controller,
      onChanged: onChanged,
      onTap: onTap,
      autofocus: autofocus,
      variant: JojuhuTextFieldVariant.filled,
    );
  }
}