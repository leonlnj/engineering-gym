# Exercise 8 - Bloom Filters

This exercise introduces **Bloom filters** using Redis and demonstrates how they are used
to efficiently test whether an element has been seen before.

## Bloom Filters Concepts

A Bloom filter is a probabilistic data structure designed to answer one question: “Have I probably seen this value before?”

Bloom filters provide:
- Guaranteed no false negatives  
  If it says no, the item has definitely not been seen.
- Possible false positives  
  If it says yes, the item might have been seen before.
- High memory efficiency and constant-time lookups

How Bloom Filters Work
A Bloom filter uses multiple hash functions to map elements to positions in a bit array:
- When adding an element, all corresponding bits are set to 1
- When checking for existence, if any bit is 0, the element is definitely not present
- If all bits are 1, the element might be present (hence the possibility of false positives)

The probability of false positives can be tuned by adjusting the filter size and number of hash functions.

### Common real-world use cases
- Preventing duplicate email signups
- URL deduplication in web crawlers
- Event or message deduplication in streaming systems
- Cache penetration protection (checking before hitting a database)
- Password breach detection (checking if password appears in known breaches)
- Network routers for packet filtering

### Redis Bloom Filters

Redis provides Bloom filter support via the [RedisBloom](https://redis.io/docs/latest/develop/data-types/probabilistic/bloom-filter/) module.

### Common Redis Bloom Filter commands

```sh
BF.RESERVE   Create a Bloom filter with specified parameters
BF.ADD       Add a single item to the filter
BF.EXISTS    Check if an item may exist in the filter
BF.MADD      Add multiple items at once
BF.MEXISTS   Check multiple items at once
BF.INFO      Get information about a Bloom filter
BF.INSERT    Add items, creating the filter if it doesn't exist
```

## Setup

- Install a bitnami redis helm chart

```sh
# Install Redis-stack and not bitnami Redis as it doesn't come with BloomFilters
helm repo add redis-stack https://redis-stack.github.io/helm-redis-stack/
helm install ex-8 redis-stack/redis-stack
```

## Test

```sh
# Connect to the redis-stack pod and run redis-cli
kubectl exec -it $(kubectl get pods -l app=redis-stack -o jsonpath='{.items[0].metadata.name}') -- redis-cli
# Create a Bloom filter named emails with a 0.1% false positive rate and an expected capacity of 1,000,000 items
BF.RESERVE emails 0.001 1000000
# Add an email (returns 1 if probably not seen before)
BF.ADD emails "user@example.com"
# Check if email exists (returns 1 if might exist)
BF.EXISTS emails "user@example.com"
# Check a new email (returns 0 if definitely doesn't exist)
BF.EXISTS emails "user1@example.com"
```

## Cleanup

```sh
helm uninstall ex-8
```
