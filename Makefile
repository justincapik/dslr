# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: adelille <adelille@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2020/11/30 19:21:49 by adelille          #+#    #+#              #
#    Updated: 2024/10/13 19:34:24 by adelille         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME_PART_1 =					analyze
NAME_PART_1_DESCRIBE =			describe

NAME_PART_2 =					visualize
NAME_PART_2_HISTOGRAM =			histogram
NAME_PART_2_SCATTER_PLOT =		scatter_plot
NAME_PART_2_PAIR_PLOT =			pair_plot

NAME_PART_3 =					logistic_regression
NAME_PART_3_TRAIN =				train
NAME_PART_3_TRAIN_FINAL_BIN =	logreg_train
NAME_PART_3_PREDICT =			predict
NAME_PART_3_PREDICT_FINAL_BIN =	logreg_predict

NAME_MODEL =					model.csv
NAME_RESULT =					houses.csv

CC =	cargo
RM = 	rm -rf

CCFLAGS = --release

# **************************************************************************** #
#	MAKEFILE	#

# MAKEFLAGS += --silent

SHELL := bash

# **************************************************************************** #
#	BASH	#

define build
	@printf "\033[1;35m$(2)\033[0m\n"

	$(CC) build $(CCFLAGS) --bin $(1)

	@cp target/release/$(1) $(2) || true
endef

# *************************************************************************** #
#	RULES	#

all:		$(NAME_PART_1) $(NAME_PART_2) $(NAME_PART_3)

$(NAME_PART_1):
	$(call build,$(NAME_PART_1_DESCRIBE),$(NAME_PART_1_DESCRIBE))

$(NAME_PART_2):
	$(call build,$(NAME_PART_2_HISTOGRAM),$(NAME_PART_2_HISTOGRAM))
	$(call build,$(NAME_PART_2_SCATTER_PLOT),$(NAME_PART_2_SCATTER_PLOT))
	$(call build,$(NAME_PART_2_PAIR_PLOT),$(NAME_PART_2_PAIR_PLOT))

$(NAME_PART_3):
	$(call build,$(NAME_PART_3_TRAIN),$(NAME_PART_3_TRAIN_FINAL_BIN))
	$(call build,$(NAME_PART_3_PREDICT),$(NAME_PART_3_PREDICT_FINAL_BIN))

check:
	$(CC) clippy --all-features -- -D warnings
	$(CC) fmt --check
	$(CC) test --all-features
	
clean:
	$(CC) clean

fclean:		clean
	@$(RM) \
		$(NAME_PART_1_DESCRIBE) \

		$(NAME_PART_2_HISTOGRAM) \
		$(NAME_PART_2_SCATTER_PLOT) \
		$(NAME_PART_2_PAIR_PLOT) \

		$(NAME_PART_3_TRAIN_FINAL_BIN) \
		$(NAME_PART_3_PREDICT_FINAL_BIN) \

		$(NAME_MODEL) \
		$(NAME_RESULT)

re:			fclean all

.PHONY: all clean fclean re check

# **************************************************************************** #
