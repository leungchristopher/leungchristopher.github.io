/* Five autonomous flows, integrated in C with equal-time RK4 samples.
 * Model references and the binary format are documented in README.md. */
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define SYSTEMS 5
#define TRAJECTORIES 4
#define STEPS 2400
#define SAMPLE_DT .01
#define INTEGRATION_DT .001
#define WIDTH 240
#define HEIGHT 210

typedef struct { double x,y,z; } State;
typedef struct { double x,y; } Point;
typedef struct {
    const char *slug,*title;
    double rate,matrix[6];
    State seed;
} Model;
static const Model models[SYSTEMS]={
    {"halvorsen","Halvorsen",1.8,{.70710678,-.70710678,0,.40824829,.40824829,-.81649658},{-1.48,-1.51,2.04}},
    {"lorenz","Lorenz",2.2,{1,0,0,0,0,1},{1,1,1}},
    {"aizawa","Aizawa",5.0,{.94,.34,0,.2,-.55,.82},{.1,0,0}},
    {"chen-celikovsky","Chen-Celikovsky",1.8,{1,0,0,0,0,1},{1,1,30}},
    {"double-scroll","Chua double-scroll",3.75,{.9965145773,-.0834188067,0,.0834188067,.9965145773,0},{.1,.005,.003}}
};
static Point points[TRAJECTORIES][STEPS+1];
static State terminal[TRAJECTORIES];
static double view_scale,view_x,view_y;

static State flow(int model,State s){
    double x=s.x,y=s.y,z=s.z;
    switch(model){
        case 0:return (State){-1.4*x-4*y-4*z-y*y,-1.4*y-4*z-4*x-z*z,-1.4*z-4*x-4*y-x*x};
        case 1:return (State){10*(y-x),x*(28-z)-y,x*y-8.0/3*z};
        case 2:return (State){(z-.7)*x-3.5*y,3.5*x+(z-.7)*y,.6+.95*z-z*z*z/3-(x*x+y*y)*(1+.25*z)+.1*z*x*x*x};
        case 3:return (State){36*(y-x),-x*z+17*y,x*y-3*z};
        default:{
            double f=-5.0/7*x-3.0/14*(fabs(x+1)-fabs(x-1));
            return (State){15.6*(y-x-f),x-y+z,-28*y};
        }
    }
}
static State add(State a,State b,double h){return (State){a.x+h*b.x,a.y+h*b.y,a.z+h*b.z};}
static State step(int model,State s,double h){
    State a=flow(model,s),b=flow(model,add(s,a,h/2)),c=flow(model,add(s,b,h/2)),d=flow(model,add(s,c,h));
    return add(s,(State){a.x+2*b.x+2*c.x+d.x,a.y+2*b.y+2*c.y+d.y,a.z+2*b.z+2*c.z+d.z},h/6);
}
static int bounded(State s){return isfinite(s.x)&&isfinite(s.y)&&isfinite(s.z)&&fabs(s.x)<100&&fabs(s.y)<100&&fabs(s.z)<100;}
static Point project(int model,State s){
    const double *m=models[model].matrix;
    if(model==4)s=(State){s.x-.45*tanh(s.x/.6),.36*(s.x+s.z+2.6*s.y),0};
    return (Point){m[0]*s.x+m[1]*s.y+m[2]*s.z,m[3]*s.x+m[4]*s.y+m[5]*s.z};
}
static int simulate(int model){
    double xmin=1e9,xmax=-1e9,ymin=1e9,ymax=-1e9;
    for(int t=0;t<TRAJECTORIES;++t){
        State s=models[model].seed;
        s.x+=t*.017;s.y+=t*.009;s.z+=t*.013;
        for(int j=0;j<30000;++j){s=step(model,s,INTEGRATION_DT);if(!bounded(s))return 1;}
        for(int j=0;j<=STEPS;++j){
            Point p=project(model,s);points[t][j]=p;
            xmin=fmin(xmin,p.x);xmax=fmax(xmax,p.x);ymin=fmin(ymin,p.y);ymax=fmax(ymax,p.y);
            if(j==STEPS)terminal[t]=s;
            for(int k=0;k<10;++k){s=step(model,s,INTEGRATION_DT);if(!bounded(s))return 1;}
        }
    }
    if(xmax-xmin<.1||ymax-ymin<.1)return 1;
    view_scale=fmin((WIDTH-40)/(xmax-xmin),(HEIGHT-40)/(ymax-ymin));
    view_x=(xmin+xmax)/2;view_y=(ymin+ymax)/2;
    for(int t=0;t<TRAJECTORIES;++t)for(int j=0;j<=STEPS;++j){
        points[t][j].x=WIDTH/2.0+(points[t][j].x-view_x)*view_scale;
        points[t][j].y=HEIGHT/2.0-(points[t][j].y-view_y)*view_scale;
    }
    return 0;
}
static int check(void){
    for(int model=0;model<SYSTEMS;++model){
        State a=models[model].seed,b=a;
        for(int k=0;k<50;++k)a=step(model,a,INTEGRATION_DT);
        for(int k=0;k<100;++k)b=step(model,b,INTEGRATION_DT/2);
        if(hypot(hypot(a.x-b.x,a.y-b.y),a.z-b.z)>1e-6||simulate(model))return 1;
        for(int t=0;t<TRAJECTORIES;++t){
            Point last=project(model,terminal[t]);
            for(int k=0;k<200000;++k){
                terminal[t]=step(model,terminal[t],INTEGRATION_DT);
                if(!bounded(terminal[t]))return 1;
                Point p=project(model,terminal[t]);
                if(fabs((p.x-view_x)*view_scale)>WIDTH/2.0-2||fabs((p.y-view_y)*view_scale)>HEIGHT/2.0-2){
                    fprintf(stderr,"%s: continuing flow leaves viewport\n",models[model].slug);return 1;
                }
            }
            Point next=project(model,terminal[t]);
            if(hypot(last.x-next.x,last.y-next.y)<1e-8)return 1;
        }
        printf("%s: bounded continuing flow, viewport coverage, short-time RK4 refinement passed.\n",models[model].slug);
    }
    return 0;
}
static void word(FILE *f,uint32_t u){for(int k=0;k<4;++k)fputc((int)((u>>(8*k))&255),f);}
static void scalar(FILE *f,float x){uint32_t u;memcpy(&u,&x,4);word(f,u);}
static void real(FILE *f,double x){uint64_t u;memcpy(&u,&x,8);word(f,(uint32_t)u);word(f,(uint32_t)(u>>32));}
static int generate(const char *directory){
    char path[1024];
    if(snprintf(path,sizeof(path),"%s/attractors.bin",directory)>=(int)sizeof(path))return 1;
    FILE *f=fopen(path,"wb");if(!f){perror(path);return 1;}
    fwrite("ATP1",1,4,f);word(f,SYSTEMS);word(f,TRAJECTORIES);word(f,STEPS+1);scalar(f,SAMPLE_DT);scalar(f,INTEGRATION_DT);
    for(int model=0;model<SYSTEMS;++model){
        if(simulate(model)){fclose(f);return 1;}
        word(f,model);scalar(f,(float)models[model].rate);
        for(int k=0;k<6;++k)real(f,models[model].matrix[k]);
        real(f,view_scale);real(f,view_x);real(f,view_y);
        for(int t=0;t<TRAJECTORIES;++t)for(int j=0;j<=STEPS;++j){scalar(f,(float)points[t][j].x);scalar(f,(float)points[t][j].y);}
        for(int t=0;t<TRAJECTORIES;++t){real(f,terminal[t].x);real(f,terminal[t].y);real(f,terminal[t].z);}
        if(snprintf(path,sizeof(path),"%s/%s.svg",directory,models[model].slug)>=(int)sizeof(path)){fclose(f);return 1;}
        FILE *svg=fopen(path,"w");if(!svg){perror(path);fclose(f);return 1;}
        fprintf(svg,"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 %d %d\" width=\"%d\" height=\"%d\" role=\"img\" aria-labelledby=\"title\"><title id=\"title\">%s attractor</title><g fill=\"none\" stroke=\"#30343b\" stroke-width=\"1\" stroke-opacity=\".35\" stroke-linecap=\"round\" stroke-linejoin=\"round\">",WIDTH,HEIGHT,WIDTH,HEIGHT,models[model].title);
        for(int t=0;t<TRAJECTORIES;++t){
            fputs("<path d=\"",svg);
            for(int j=0;j<=1200;j+=2)fprintf(svg,"%c%.2f %.2f",j?'L':'M',points[t][j].x,points[t][j].y);
            fputs("\"/>",svg);
        }
        fputs("</g></svg>\n",svg);
        int failed=ferror(svg);if(fclose(svg))failed=1;if(failed){fclose(f);return 1;}
        printf("Generated %s.\n",models[model].slug);
    }
    int failed=ferror(f);if(fclose(f))failed=1;return failed;
}
int main(int argc,char **argv){
    if(argc==2&&!strcmp(argv[1],"--check"))return check();
    if(argc==3&&!strcmp(argv[1],"--generate"))return generate(argv[2]);
    fputs("Usage: attractors --check | --generate assets/graphics\n",stderr);return 1;
}
