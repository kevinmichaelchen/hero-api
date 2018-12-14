import React from 'react';
import { connect } from 'react-redux';
import { createStructuredSelector } from 'reselect';
import { bindActionCreators } from 'redux';
import { Formik, Form, Field, ErrorMessage } from 'formik';
import {
  ActionCreators as HeroesActionCreators,
  Selectors as HeroesSelectors,
} from '../duck';

class CreateHero extends React.Component {
  render() {
    const {
      actions: { createHeroRequest },
    } = this.props;
    return (
      <div>
        <h1>Create Hero</h1>
        <Formik
          initialValues={{ name: '', identity: '', hometown: '', age: '' }}
          validate={values => {
            let errors = {};
            if (!values.name) {
              errors.name = 'Required';
            }
            if (!values.identity) {
              errors.identity = 'Required';
            }
            if (!values.hometown) {
              errors.hometown = 'Required';
            }
            if (!values.age) {
              errors.age = 'Required';
            }
            return errors;
          }}
          onSubmit={(values, { setSubmitting }) => {
            setTimeout(() => {
              createHeroRequest(values);
              setSubmitting(false);
            }, 400);
          }}
        >
          {({ isSubmitting }) => (
            <Form>
              <div>
                <span>Name:</span>
                <Field type="name" name="name" />
                <ErrorMessage name="name" component="div" />
              </div>

              <div>
                <span>Identity:</span>
                <Field type="identity" name="identity" />
                <ErrorMessage name="identity" component="div" />
              </div>

              <div>
                <span>Hometown:</span>
                <Field type="hometown" name="hometown" />
                <ErrorMessage name="hometown" component="div" />
              </div>

              <div>
                <span>Age:</span>
                <Field type="age" name="age" />
                <ErrorMessage name="age" component="div" />
              </div>

              <button type="submit" disabled={isSubmitting}>
                Submit
              </button>
            </Form>
          )}
        </Formik>
      </div>
    );
  }
}

const mapStateToProps = createStructuredSelector({
  loading: HeroesSelectors.loadingSelector,
  error: HeroesSelectors.errorSelector,
});

const mapDispatchToProps = dispatch => ({
  actions: bindActionCreators(
    {
      createHeroRequest: HeroesActionCreators.createHeroRequest,
    },
    dispatch
  ),
});

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(CreateHero);
