from faker import Faker
import random

fake = Faker()

def generate_prompts(num_prompts):
    prompts = []
    for _ in range(num_prompts):
        prompt_length = random.randint(10, 100)
        prompt = fake.sentence(nb_words=prompt_length).strip()
        prompts.append(prompt)
    return prompts

prompts = generate_prompts(3100)

with open('prompts.txt', 'w') as f:
    for prompt in prompts:
        f.write(prompt + '\n')

print(f"Generated {len(prompts)} prompts and saved to prompts.txt")
