import 'package:flutter_animate/flutter_animate.dart';
import 'package:flutter/material.dart';
import '../theme/tokens.dart';

/// Jojuhu Animation Presets
/// Reusable animation effects using flutter_animate

class JojuhuAnimations {
  static Effect fadeSlideUp({Duration? duration}) {
    return Effect(
      fadeIn(duration: duration ?? JojuhuDuration.medium,
          curve: JojuhuAnimation.easeOutCubic) +
          slideY(
            begin: 20,
            end: 0, duration: duration ?? JojuhuDuration.medium,
            curve: JojuhuAnimation.easeOutCubic,
          ),
    );
  }

  static Effect fadeSlideIn({Duration? duration}) {
    return Effect(
      fadeIn(duration: duration ?? JojuhuDuration.medium,
          curve: JojuhuAnimation.easeOutCubic) +
          slideX(
            begin: -20,
            end: 0, duration: duration ?? JojuhuDuration.medium,
            curve: JojuhuAnimation.easeOutCubic,
          ),
    );
  }

  static Effect scaleOnTap() {
    return Effect(
      scale(
        begin: const Offset(1, 1),
        end: const Offset(0.95, 0.95), duration: JojuhuDuration.quick,
        curve: JojuhuAnimation.easeOut,
      ),
    );
  }

  static Effect heartBeat() {
    return Effect(
      scale(
        begin: const Offset(1, 1),
        end: const Offset(1.2, 1.2), duration: JojuhuDuration.quick,
        curve: JojuhuAnimation.easeOutCubic,
      ) +
          then() +
          scale(
            begin: const Offset(1.2, 1.2),
            end: const Offset(1, 1), duration: JojuhuDuration.quick,
            curve: JojuhuAnimation.easeOutCubic,
          ),
    );
  }

  static Effect shimmer() {
    return Effect(
      shimmer(
        duration: 1500.ms,
        color: Colors.white.withOpacity(0.3),
        angle: 0.0,
      ),
    );
  }

  static Effect pulse() {
    return Effect(
      fadeIn(duration: JojuhuDuration.quick) +
          scale(
            begin: const Offset(0.95, 0.95),
            end: const Offset(1, 1), duration: JojuhuDuration.medium,
            curve: JojuhuAnimation.elasticOut,
          ),
    );
  }

  static Effect fadeIn() {
    return Effect(
      fadeIn(duration: JojuhuDuration.medium,
          curve: JojuhuAnimation.easeOutCubic),
    );
  }

  static Effect staggeredList(int index) {
    return Effect(
      fadeIn(duration: JojuhuDuration.medium,
          curve: JojuhuAnimation.easeOutCubic) +
          slideY(
            begin: 30,
            end: 0, duration: JojuhuDuration.medium,
            curve: JojuhuAnimation.easeOutCubic,
            delay: Duration(milliseconds: 50 * index),
          ),
    );
  }
}

extension AnimateWidgetExtension on Widget {
  Widget fadeSlideUp({Duration? delay}) {
    return animate(delay: delay ?? Duration.zero)
        .fadeIn(duration: JojuhuDuration.medium)
        .slideY(begin: 0.1, end: 0, duration: JojuhuDuration.medium);
  }

  Widget scaleOnTap() {
    return animate().scale(
      begin: const Offset(1, 1),
      end: const Offset(0.95, 0.95),
      duration: JojuhuDuration.quick,
    );
  }

  Widget heartBeat() {
    return animate()
        .scale(
          begin: const Offset(1, 1),
          end: const Offset(1.15, 1.15),
          duration: JojuhuDuration.quick,
        )
        .then()
        .scale(
          begin: const Offset(1.15, 1.15),
          end: const Offset(1, 1),
          duration: JojuhuDuration.quick,
        );
  }

  Widget staggerListItem(int index) {
    return animate(delay: Duration(milliseconds: 50 * index))
        .fadeIn(duration: JojuhuDuration.medium)
        .slideY(begin: 0.2, end: 0, duration: JojuhuDuration.medium);
  }
}