interface FeatureCardProps {
  emoji: string;
  title: string;
  description: string;
}

export function FeatureCard(props: FeatureCardProps) {
  return (
    <div class="p-6 rounded-lg bg-gray-50 dark:bg-gray-800">
      <h3 class="text-xl font-semibold mb-2">
        {props.emoji} {props.title}
      </h3>
      <p class="text-gray-600 dark:text-gray-400">{props.description}</p>
    </div>
  );
}
