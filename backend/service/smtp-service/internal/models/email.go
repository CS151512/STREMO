package models

type EmailTask struct {
	To           string            `json:"to" binding:"required,email"`
	Subject      string            `json:"subject" binding:"required"`
	TemplateName string            `json:"template_name" binding:"required"`
	TemplateData map[string]string `json:"template_data"`
}
