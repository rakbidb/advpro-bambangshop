# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## Mandatory Checklists (Publisher)
-   [✅] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [✅] Commit: `Create Subscriber model struct.`
    -   [✅] Commit: `Create Notification model struct.`
    -   [✅] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [✅] Commit: `Implement add function in Subscriber repository.`
    -   [✅] Commit: `Implement list_all function in Subscriber repository.`
    -   [✅] Commit: `Implement delete function in Subscriber repository.`
    -   [✅] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [✅] Commit: `Create Notification service struct skeleton.`
    -   [✅] Commit: `Implement subscribe function in Notification service.`
    -   [✅] Commit: `Implement subscribe function in Notification controller.`
    -   [✅] Commit: `Implement unsubscribe function in Notification service.`
    -   [✅] Commit: `Implement unsubscribe function in Notification controller.`
    -   [✅] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

    In BambangShop case, assuming that there is only one type of Subscriber, a single Model struct is enough. However, if we refer to the Dependency Inversion Principle and Open-Closed Principle, it is actually better to make interface for Subscriber, because according to the DIP the code must depend on abstraction. Also, if there will be multiple type of Subscriber, abstraction (interface) is better to maintain according to the Open-Closed Principle.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

    Based on my understanding, it is necessary to use DashMap because it will be easier to access the data using DashMap instead of using a Vec. Most of the operation using map/dictionary-like data structure is constant, so it will be better performance wise.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

    In BambangShop case, both DashMap and Singleton Pattern is used. Singleton is used because we don't need multiple database/repository instance for the app. DashMap is used because it is a data structure that is thread safe and is suitable for multithreading.

#### Reflection Publisher-2

1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

    We need to separate service and repository from a model because it aligns with the Single Responsibility Principle (SRP) and also Open-Closed Principle. By separating service and repository, each module has its own responsibility. Also, if we don't separate them it will be harder to modify the code without breaking the code (Open-Closed Principle)


2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

    The code will be more complex since it will contain of the combination of model, service, and repository in a file/module. The code's maintainability and reusability will decrease and will violate the SOLID principle. 

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

    Postman is a very helpful tool to test the backend part of a project. I have known and used Postman since last semester and will most likely keep using it for future projects. I can test the API easily, make and save a custom configuration (like which url or which HTTP method used)

#### Reflection Publisher-3
